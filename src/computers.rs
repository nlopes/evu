use std::fs;

use crate::error::Result;
use crate::utils;

use arq::computer::ComputerInfo;

pub fn get_computers(path: &str) -> Result<Vec<ComputerInfo>> {
    let mut computers = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry.unwrap();
        let reader = utils::get_file_reader(entry.path().join("computerinfo"));
        computers.push(ComputerInfo::new(reader, entry.file_name().into_string()?)?);
    }
    Ok(computers)
}

pub fn show(path: &str) -> Result<()> {
    println!("Computers\n---------");
    for computer in get_computers(path)?.iter() {
        println!(
            "> [{}] ({}@{})",
            computer.uuid, computer.user_name, computer.computer_name,
        );
    }
    Ok(())
}
