mod arghandler;
mod error;
mod loader;
pub mod user;
pub use self::error::CliError;

use gramps::gen::dbstate::DbState;
use std::error::Error;
use std::sync::Arc;

use crate::arghandler::ArgHandler;
use crate::user::User;

#[derive(Debug)]
pub struct CliManager {}

impl CliManager {
    pub fn with_loader(dbstate: Arc<DbState>, user: user::User) -> Self {
        log::trace!("Creating CliManager with loader ...");
        let loader = loader::CliDbLoader::new(dbstate.clone());
        Self {}
    }
}

pub fn startcli(clioptions: ()) -> Result<(), CliError> {
    log::trace!("startcli");
    let dbstate = Arc::new(DbState::new());
    let user = User::new();
    let climanager = CliManager::with_loader(dbstate.clone(), user);

    let handler = ArgHandler::new(dbstate, clioptions, climanager);

    handler.handle_args_cli()?;
    return Ok(());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
