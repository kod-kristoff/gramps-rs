use std::sync::Arc;

use gramps::gen::dbstate::DbState;

use crate::{CliError, CliManager};

#[derive(Debug)]
pub struct ArgHandler {
    dbstate: Arc<DbState>,
    smgr: CliManager,
}

impl ArgHandler {
    pub fn new(dbstate: Arc<DbState>, clioptions: (), smgr: CliManager) -> Self {
        log::trace!("creating Arghandler ...");
        Self { dbstate, smgr }
    }
}

impl ArgHandler {
    pub fn handle_args_cli(&self) -> Result<(), CliError> {
        log::trace!("handle_args_cli");
        Ok(())
    }
}
