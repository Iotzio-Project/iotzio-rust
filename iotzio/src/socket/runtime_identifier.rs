use std::ops::Deref;
use std::sync::Mutex;

static ACTIVE_BOARDS: Mutex<Vec<u64>> = Mutex::new(Vec::new());

#[derive(Debug)]
pub struct RuntimeIdentifier(u64);

impl RuntimeIdentifier {
    pub fn new(inner: u64) -> Option<RuntimeIdentifier> {
        let mut active_boards = ACTIVE_BOARDS.lock().unwrap();

        match active_boards.contains(&inner) {
            true => None,
            false => {
                active_boards.push(inner);

                Some(RuntimeIdentifier(inner))
            }
        }
    }
}

impl Drop for RuntimeIdentifier {
    fn drop(&mut self) {
        let mut active_boards = ACTIVE_BOARDS.lock().unwrap();

        active_boards.retain(|x| x.ne(&self.0))
    }
}

impl Deref for RuntimeIdentifier {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
