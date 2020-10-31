mod args;
mod error;
mod git;
mod componentgen;
mod projectgen;

async fn run() -> Result<(), error::MineError> {
    let args = args::get_args();
    return match args.subcommand_name() {
        Some(args::ARG_NEW) => {
            projectgen::generate(args.subcommand_matches(args::ARG_NEW).unwrap().clone()).await
        }
        Some(args::ARG_COMPONENT) => {
            componentgen::generate(
                args.subcommand_matches(args::ARG_COMPONENT)
                    .unwrap()
                    .clone(),
            )
            .await
        }
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
