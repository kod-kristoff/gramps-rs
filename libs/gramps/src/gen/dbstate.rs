#[derive(Debug)]
pub struct DbState {}

impl DbState {
    pub fn new() -> Self {
        log::trace!("Creating DbState ...");
        Self {}
    }
}
