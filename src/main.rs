use std::error::Error;

fn main() {
    let mut builder = env_logger::Builder::from_default_env();

    builder
        .format_timestamp(None)
        .filter(None, log::LevelFilter::Trace)
        .init();

    println!("Hello, world!");
    let errors = run();
    if errors.len() > 0 {
        for error in errors {
            log::error!("{}", error);
        }
    }
}

fn run() -> Vec<Box<dyn Error>> {
    log::trace!("run");
    let errors = Vec::new();
    errors
}
