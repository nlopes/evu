extern crate evu;

fn main() -> Result<(), evu::error::Error> {
    let matches = evu::cli::parse_flags();

    match matches.subcommand() {
        ("show", Some(cmd)) => match cmd.subcommand() {
            ("computers", Some(_)) => evu::computers::show(matches.value_of("path").unwrap())?,
            ("folders", Some(c)) => {
                evu::folders::show(matches.value_of("path").unwrap(), c.value_of("computer").unwrap())?
            }
            ("tree", Some(c)) => evu::tree::show(
                matches.value_of("path").unwrap(),
                c.value_of("computer").unwrap(),
                c.value_of("folder").unwrap(),
            )?,
            _ => (),
        },
        ("restore", Some(cmd)) => evu::recovery::restore_file(
            matches.value_of("path").unwrap(),
            cmd.value_of("computer").unwrap(),
            cmd.value_of("folder").unwrap(),
            cmd.value_of("FILEPATH").unwrap(),
        )?,
        _ => (),
    }
    Ok(())
}
