use std::marker::PhantomData;
use std::time::Duration;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Delay {
    phantom_data: PhantomData<()>,
}

impl Default for Delay {
    fn default() -> Self {
        Delay {
            phantom_data: Default::default(),
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl embedded_hal::delay::DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32) {
        std::thread::sleep(Duration::from_nanos(ns as u64))
    }

    fn delay_us(&mut self, us: u32) {
        std::thread::sleep(Duration::from_micros(us as u64))
    }

    fn delay_ms(&mut self, ms: u32) {
        std::thread::sleep(Duration::from_millis(ms as u64))
    }
}

impl embedded_hal_async::delay::DelayNs for Delay {
    async fn delay_ns(&mut self, ns: u32) {
        async_std::task::sleep(Duration::from_nanos(ns as u64)).await
    }

    async fn delay_us(&mut self, us: u32) {
        async_std::task::sleep(Duration::from_micros(us as u64)).await
    }

    async fn delay_ms(&mut self, ms: u32) {
        async_std::task::sleep(Duration::from_millis(ms as u64)).await
    }
}
