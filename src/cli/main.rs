use crate::cli::options::{Args, Subcommands};
use clap::Parser;
use gramps::gen::dbstate::DbState;
use gramps_core as core;
use grampscli::{user::User, CliManager};

// fn main() {
//     let mut builder = env_logger::Builder::from_default_env();

//     builder
//         .format_timestamp(None)
//         .filter(None, log::LevelFilter::Trace)
//         .init();

//     if let Err(err) = cli::main() {
//         log::error!("{}", err);
//     }
// }

pub fn main() -> eyre::Result<()> {
    log::trace!("cli::main");
    let args: Args = Args::parse();
    // let errors = Vec::new();
    // let dbstate = Arc::new(DbState::new());
    // let user = User::new();
    // let climanager = CliManager::with_loader(dbstate, user);
    // errors
    let db_path = args.db_path;
    match args.cmd {
        Subcommands::Init => core::database::init(db_path),
    }
    let clioptions = ();
    grampscli::startcli(clioptions)?;
    Ok(())
}
