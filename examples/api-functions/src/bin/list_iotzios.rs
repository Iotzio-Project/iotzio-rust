#[async_std::main]
pub async fn main() {
    let iotzio_infos = iotzio::IotzioManager::new().list_connected_boards_async().await.unwarp();

    for iotzio_info in iotzio_infos {
        let iotzio = iotzio_info.open_async().await.unwarp();

        println!("Found Iotzio {0} with serial number {1}!", iotzio.version(), iotzio.serial_number());
    }
}
