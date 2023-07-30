use gramps::gen::dbstate::DbState;
use std::sync::Arc;

pub struct CliDbLoader {
    dbstate: Arc<DbState>,
}

impl CliDbLoader {
    pub fn new(dbstate: Arc<DbState>) -> Self {
        log::trace!("Creating CliDbLoader ...");
        Self { dbstate }
    }
}
