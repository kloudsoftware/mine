mod args;
mod componentgen;
mod error;
mod projectgen;

fn main() {
    let args = args::get_args();

    match args.subcommand_name() {
        Some("new") => projectgen::generate(),
        Some("component") => componentgen::generate(),
        _ => Err(error::MineError::InvalidArgument),
    }
    .unwrap();
}
