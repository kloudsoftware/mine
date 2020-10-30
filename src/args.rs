use clap::{App, AppSettings};

pub fn get_args() -> clap::ArgMatches<'static> {
    App::new("mine")
        .version("0.1b1")
        .author("github.com/kloudsoftware")
        .about("CLI tool to bootstrap eisen projects in an instant")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommands(vec![
            App::new("new").about("generate a new eisen project"),
            App::new("component").about("generate a new eisen component"),
        ])
        .get_matches()
}
