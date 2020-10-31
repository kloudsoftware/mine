use clap::{App, AppSettings, Arg};

pub const ARG_NEW: &str = "new";
pub const ARG_PROJECT_NAME: &str = "name";
pub const ARG_COMPONENT: &str = "component";

pub fn get_args() -> clap::ArgMatches<'static> {
    App::new("mine")
        .version("0.1b1")
        .author("github.com/kloudsoftware")
        .about("CLI tool to bootstrap eisen projects in an instant")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommands(vec![
            App::new(ARG_NEW).about("generate a new eisen project").arg(
                Arg::with_name(ARG_PROJECT_NAME)
                    .help("name of the new project")
                    .required(true),
            ),
            App::new(ARG_COMPONENT).about("generate a new eisen component"),
        ])
        .get_matches()
}
