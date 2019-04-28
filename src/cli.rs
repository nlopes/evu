use clap;

pub fn parse_flags<'a>() -> clap::ArgMatches<'a> {
    clap::App::new("evu")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Command line interface to ARQ")
        .arg(clap::Arg::from_usage("-p --path [path]   'Path to the backup folder'").global(true))
        .subcommand(
            clap::SubCommand::with_name("show")
                .about("display one or more resources")
                .subcommand(clap::SubCommand::with_name("computers").about("show computers"))
                .subcommand(
                    clap::SubCommand::with_name("folders")
                        .about("show folders")
                        .args_from_usage("-c --computer [computer]   'Computer UUID'"),
                )
                .subcommand(
                    clap::SubCommand::with_name("tree")
                        .about("show tree")
                        .args_from_usage(
                            "-c --computer [computer]   'Computer UUID'
                             -f --folder [folder]   'Folder UUID'",
                        ),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("restore")
                .about("restore file")
                .args_from_usage(
                    "-c --computer [computer]   'Computer UUID'
                     -f --folder [folder]   'Folder UUID'
                     <FILEPATH> 'Absolute path to restore'",
                ),
        )
        .get_matches()
}
