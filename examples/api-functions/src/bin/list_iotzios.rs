#[async_std::main]
pub async fn main() {
    let iotzio_infos = iotzio::IotzioManager::new().list_connected_boards_async().await.unwrap();

    if iotzio_infos.is_empty() {
        println!("No Iotzio found!");
    } else {
        for iotzio_info in iotzio_infos {
            let iotzio = iotzio_info.open_async().await.unwrap();

            println!("Found Iotzio {} with serial number {}!", iotzio.version(), iotzio.serial_number());
        }
    }
}
