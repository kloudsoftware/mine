mod args;
mod componentgen;
mod error;
mod projectgen;

async fn run() -> Result<(), error::MineError> {
    let args = args::get_args();

    return match args.subcommand_name() {
        Some("new") => projectgen::generate().await,
        Some("component") => componentgen::generate().await,
        _ => Err(error::MineError::IllegalArgumentConfiguration),
    };
}

fn main() {
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    let res = runtime.block_on(run());
    if res.is_err() {
        eprintln!("{}", res.unwrap_err());
        std::process::exit(1);
    };
}
