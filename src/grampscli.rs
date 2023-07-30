mod cli;

// use std::error::Error;

// use gramps::gen::dbstate::DbState;
// use grampscli::{user::User, CliManager};
// use std::sync::Arc;

fn main() {
    let mut builder = env_logger::Builder::from_default_env();

    builder
        .format_timestamp(None)
        .filter(None, log::LevelFilter::Trace)
        .init();

    if let Err(err) = cli::main() {
        log::error!("{}", err);
    }
}

// fn run() -> eyre::Result<()> {
//     log::trace!("run");
//     // let errors = Vec::new();
//     // let dbstate = Arc::new(DbState::new());
//     // let user = User::new();
//     // let climanager = CliManager::with_loader(dbstate, user);
//     // errors
//     let clioptions = ();
//     grampscli::startcli(clioptions)?;
//     Ok(())
// }
