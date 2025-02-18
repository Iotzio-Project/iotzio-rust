use log::info;
use log::error;
use log::LevelFilter;
use std::future::Future;

pub async fn run_with_logger<F, Fut>(level_filter: LevelFilter, function: F) where
    F: Fn() -> Fut,
    Fut: Future<Output = anyhow::Result<()>>, {
    simple_logger::SimpleLogger::new().with_level(level_filter).without_timestamps().env().init().unwrap();

    match function.await {
        Ok(_) => {
            info!("Done.");
        }
        Err(x) => {
            error!("{}" , x);
        }
    }
}