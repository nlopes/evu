use std;
use std::path::Path;

use crate::error::Result;
use crate::utils;

use arq::folder::Folder;

pub fn show(path: &str, computer: &str) -> Result<()> {
    let computers_path = Path::new(path).join(computer);
    let master_keys = utils::get_master_keys(&path, &computer)?;

    println!("Folders\n-------");
    for entry in std::fs::read_dir(computers_path.join("buckets"))? {
        let filename = entry?.path();
        let mut reader = utils::get_file_reader(filename);
        let folder = Folder::new(&mut reader, &master_keys)?;
        println!(
            "> [{}] ({}) Computer: {}",
            folder.bucket_uuid, folder.local_path, computer
        );
    }
    Ok(())
}
