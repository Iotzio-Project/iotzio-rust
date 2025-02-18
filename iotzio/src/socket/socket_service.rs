use crate::backend::{DeviceReader, DeviceWriter};
use crate::communication::{
    Command, DeviceReport, FatalError, ProtocolError, Response, DEVICE_REPORT_HEADER_SIZE, HOST_REPORT_HEADER_SIZE,
    IOTZIO_PROTOCOL_VERSION,
};
use crate::communication::{HostReport, PROTOCOL_INFO_BUFFER_SIZE, PROTOCOL_INFO_REPORT_ID};
use crate::iotzio::InitializationError;
use crate::modules::ModuleError;
use crate::socket::{RuntimeIdentifier, Socket, SocketInput, SocketOutput};
use crate::IotzioInfo;
use async_oneshot::Receiver;
use futures_concurrency::future::Race;
use hidparser::{Report, ReportDescriptor};
use log::Level::Trace;
use log::{log_enabled, trace};
use postcard::{from_bytes, to_slice};
use std::ops::DerefMut;
use std::sync::atomic::{AtomicU32, Ordering};

pub async fn new_socket(iotzio_info: &IotzioInfo) -> Result<Socket, InitializationError> {
    let runtime_identifier =
        RuntimeIdentifier::new(iotzio_info.runtime_identifier).ok_or(InitializationError::DeviceAlreadyInUseError)?;

    let (mut reader, mut writer) =
        iotzio_info
            .device_info
            .open()
            .await
            .map_err(|x| InitializationError::DeviceOpenError {
                error_message: format!("Error opening Iotzio HID connection: {0}", x),
            })?;

    let (protocol_version, hid_descriptor) = get_protocol_info(&mut reader, &mut writer).await?;

    if protocol_version != IOTZIO_PROTOCOL_VERSION {
        return Err(InitializationError::MismatchingProtocolVersion {
            driver: IOTZIO_PROTOCOL_VERSION,
            board: protocol_version,
        });
    }

    let input_reports = get_reports(&hid_descriptor.input_reports)?;

    let output_reports = get_reports(&hid_descriptor.output_reports)?;

    let input_buffer_size = get_buffer_size(&input_reports)?;

    let output_buffer_size = get_buffer_size(&output_reports)?;

    let identifier = AtomicU32::new(0);

    let output = SocketOutput {
        writer,
        buffer: vec![0u8; output_buffer_size].into_boxed_slice(),
        reports: output_reports,
    }
    .into();

    let input = SocketInput {
        reader,
        buffer: vec![0u8; input_buffer_size].into_boxed_slice(),
    }
    .into();

    Ok(Socket {
        output,
        input,
        packet_counter: identifier,
        input_queue_mutex: Default::default(),
        runtime_identifier,
    })
}

pub async fn send_command(socket: &Socket, command: Command) -> Result<Result<Response, ModuleError>, FatalError> {
    let send_id = socket.packet_counter.fetch_add(1, Ordering::Relaxed);

    let host_report = HostReport {
        identifier: send_id,
        command,
    };

    write_report(socket.output.lock().await.deref_mut(), host_report).await?;

    let response_channel = enqueue_for_response(&socket, send_id).await;

    // read_and_distribute_response:
    // This method continuously waits for incoming HID reports in a loop.
    // If it receives a report with a matching send_id, it returns the report.
    // Otherwise, it places the report in a queue and continues looping.
    // This method blocks if another thread/task is already blocking.

    let future_left = read_and_distribute_response(&socket, send_id);

    // response_channel:
    // This method is used when read_and_distribute_response is already blocking.
    // It waits for the other thread/task to send the desired report.

    let future_right = async { response_channel.await.unwrap_or_else(|_| Err(FatalError::DeviceClosed)) };

    // race:
    // This function runs both methods concurrently.
    // When the first thread/task receives its report and completes,
    // the second thread/task can take its place reading reports.

    (async move { future_left.await }, async move { future_right.await })
        .race()
        .await
}

#[inline]
async fn read_and_distribute_response(
    socket: &Socket,
    send_id: u32,
) -> Result<Result<Response, ModuleError>, FatalError> {
    let mut input = socket.input.lock().await;

    loop {
        let device_report_result = read(&mut input).await;

        if log_enabled!(Trace) {
            match &device_report_result {
                Ok(x) => trace!("Received {0}", x),
                Err(x) => trace!("Received {0}", x),
            }
        }

        let response_result = device_report_result.and_then(|x| match x {
            DeviceReport::Response { identifier, result } => Ok((identifier, result)),
            DeviceReport::FatalError { error } => Err(error),
        });

        let mut input_queue = socket.input_queue_mutex.lock().await;

        match response_result {
            Ok((received_id, result)) => {
                // Get entry from input_queue for matching received_id.

                let (_, sender) = match input_queue.iter_mut().filter(|(id, _)| *id == received_id).next() {
                    None => continue,
                    Some(x) => x,
                };

                match send_id == received_id {
                    true => {
                        // We send this command and this is our response, we return this report.
                        // We do NOT send it to the queue, just remove entry from there.

                        input_queue.retain(|(id, sender)| *id != received_id && !sender.is_closed());

                        return Ok(result);
                    }
                    false => {
                        // We send a command, but this is some ones other response.
                        // We send it to the queue, and then remove the entry from there.

                        _ = sender.send(Ok(result));

                        input_queue.retain(|(id, sender)| *id != received_id && !sender.is_closed());
                    }
                }
            }
            Err(error) => {
                // Shit hits the fan. Send error to all queue entries, except entry from our self.
                // We empty input_queue and return the error.

                input_queue
                    .iter_mut()
                    .filter(|(id, _)| *id != send_id)
                    .for_each(|(_, sender)| {
                        _ = sender.send(Err(error.clone()));
                    });

                input_queue.clear();

                return Err(error);
            }
        }
    }
}

#[inline]
async fn enqueue_for_response(
    socket: &Socket,
    send_id: u32,
) -> Receiver<Result<Result<Response, ModuleError>, FatalError>> {
    let mut input_queue = socket.input_queue_mutex.lock().await;

    let (tx, rx) = async_oneshot::oneshot();

    input_queue.push((send_id, tx));

    drop(input_queue);

    rx
}

#[inline]
async fn write_report(output: &mut SocketOutput, host_report: HostReport) -> Result<(), FatalError> {
    trace!("Send {0}", &host_report);

    output.buffer.fill(0x00);

    // Send a new report. A host report has the following format:
    // Report ID (u8), Identifier (u32), Command ID (u16), Postcard Payload ([u8])

    let write_buffer_size = output.buffer.len();

    let mut report_length = 1usize;

    {
        let identifier = host_report.identifier.to_le_bytes();

        output.buffer[report_length..report_length + identifier.len()].copy_from_slice(&identifier);
        report_length += identifier.len();
    }

    {
        let command_id = host_report.command.id().to_le_bytes();

        output.buffer[report_length..report_length + command_id.len()].copy_from_slice(&command_id);
        report_length += command_id.len();
    }

    debug_assert_eq!(report_length, HOST_REPORT_HEADER_SIZE);

    report_length += to_slice(
        &host_report.command,
        &mut output.buffer[HOST_REPORT_HEADER_SIZE..write_buffer_size - 1],
    )
    .map(|x| x.len())?;

    let (report_id, report_count) = get_report_id(&output.reports, report_length)?;

    output.buffer[0] = report_id;

    output
        .writer
        .write_output_report(&mut output.buffer[..report_count + 1]) // Report Count + Report ID
        .await
        .map_err(|x| FatalError::read_error(format!("Error writing data to Iotzio device: {0}", x)))
}

#[inline]
fn get_report_id(output_reports: &Vec<(u8, usize)>, payload_size: usize) -> Result<(u8, usize), FatalError> {
    let report_id = output_reports
        .iter()
        .filter(|x| payload_size <= x.1)
        .next()
        .map(|x| x.clone())
        .ok_or(ProtocolError::ErrorSelectingReportId)?;

    Ok(report_id)
}

#[inline]
async fn read(input: &mut SocketInput) -> Result<DeviceReport, FatalError> {
    input.buffer.fill(0x00);

    let report_length = input
        .reader
        .read_input_report(&mut input.buffer)
        .await
        .map_err(|x| FatalError::read_error(format!("Error reading data from Iotzio device: {0}", x)))?;

    if report_length <= DEVICE_REPORT_HEADER_SIZE {
        return Err(FatalError::from(ProtocolError::PacketTooSmall));
    }

    let device_report = from_bytes::<DeviceReport>(&input.buffer[1..report_length])?;

    Ok(device_report)
}

#[inline]
async fn get_protocol_info(
    reader: &mut DeviceReader,
    writer: &mut DeviceWriter,
) -> Result<(u16, ReportDescriptor), InitializationError> {
    let mut buffer = [0u8; PROTOCOL_INFO_BUFFER_SIZE];

    buffer[0] = PROTOCOL_INFO_REPORT_ID;

    writer
        .write_output_report(&mut buffer)
        .await
        .map_err(|x| InitializationError::FatalErrorWrapper {
            error: FatalError::HostWriteError {
                error_message: format!("Error writing protocol info request to Iotzio device: {0}", x),
            },
        })?;

    let report_length = reader.read_input_report(&mut buffer).await.map_err(|x| {
        FatalError::read_error(format!(
            "Error reading protocol info response from Iotzio device: {0}",
            x
        ))
    })?;

    if report_length != PROTOCOL_INFO_BUFFER_SIZE {
        return Err(InitializationError::from(FatalError::from(
            ProtocolError::PacketTooSmall,
        )));
    }

    let mut index = 1usize;

    let protocol_version = u16::from_le_bytes([buffer[index], buffer[index + 1]]);
    index += 2;

    let hid_descriptor_length = u16::from_le_bytes([buffer[index], buffer[index + 1]]) as usize;
    index += 2;

    let hid_descriptor_slice = &buffer[index..index + hid_descriptor_length];

    let hid_descriptor =
        hidparser::parse_report_descriptor(hid_descriptor_slice).map_err(|x| InitializationError::DeviceOpenError {
            error_message: format!("Received invalid Iotzio HID device descriptor: {:?}", x),
        })?;

    Ok((protocol_version, hid_descriptor))
}

fn get_reports(reports: &[Report]) -> Result<Vec<(u8, usize)>, InitializationError> {
    let mut vec: Vec<(u8, usize)> = reports
        .iter()
        .filter_map(|x| {
            let report_id = match x.report_id {
                None => return None,
                Some(x) => u32::from(x) as u8,
            };

            match report_id < PROTOCOL_INFO_REPORT_ID {
                true => {
                    let report_count = x.size_in_bits / 8usize;

                    Some((report_id, report_count))
                }
                false => None,
            }
        })
        .collect();

    vec.sort_by(|a, b| a.1.cmp(&b.1)); // Order by report size ascending.

    Ok(vec)
}

fn get_buffer_size(reports: &Vec<(u8, usize)>) -> Result<usize, InitializationError> {
    match reports.into_iter().max_by(|x, y| x.1.cmp(&y.1)) {
        None => {
            return Err(InitializationError::DeviceOpenError {
                error_message: "Received invalid Iotzio HID device descriptor.".to_string(),
            });
        }
        Some(x) => Ok(1 + x.1), // Report ID + Report Count
    }
}
