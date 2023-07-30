use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[clap(name = "grampscli", about = "Gramps CLI", version, author)]
pub struct Args {
    /// Db
    #[arg(short('d'), long("db"))]
    pub db_path: Option<PathBuf>,

    #[clap(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommands {
    /// Initialize a database in the given `DB_PATH` or `gramps.db` in the current directory.
    Init,
}
