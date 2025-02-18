#![forbid(unsafe_code)]

use iotzio::IotzioManager;
use log::{info, warn};

mod bootstrap;

#[wasm_bindgen::wasm_bindgen(start)]
pub async fn main() {
    boostrap::run_with_logger(log::LevelFilter::Info, example).await;
}

async fn example() -> anyhow::Result<()> {
    let iotzio_infos = IotzioManager::new().list_connected_boards_async().await?;

    for iotzio_info in iotzio_infos {
        let iotzio = iotzio_info.open_async().await?;

        info!("Found Iotzio {0} with serial number {1}!", iotzio.version(), iotzio.serial_number());
    }

    Ok(())
}