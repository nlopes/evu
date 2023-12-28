use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use crate::error::Result;

use arq::folder::{Folder, FolderData};
use arq::object_encryption;

pub fn get_latest_folder_data_path(path: &Path) -> Result<PathBuf> {
    let mut newest = "0".to_string();
    // TODO(nlopes): what if the path doesn't exist? Provide nicer output.
    for entry in std::fs::read_dir(path)? {
        let filename = entry?.file_name().to_str().unwrap().to_string();
        if filename > newest {
            newest = filename;
        }
    }
    Ok(path.join(newest))
}

pub fn read_arq_folder(
    path: &str,
    computer: &str,
    folder: &str,
    master_keys: Vec<Vec<u8>>,
) -> Result<Folder> {
    let path = Path::new(path).join(computer).join("buckets").join(folder);
    let mut reader = get_file_reader(path);
    Ok(Folder::new(&mut reader, &master_keys)?)
}

pub fn find_latest_folder_sha(path: &str, computer: &str, folder: &str) -> Result<String> {
    let refs_path = Path::new(path)
        .join(computer)
        .join("bucketdata")
        .join(folder)
        .join("refs");

    let folder_data_path = get_latest_folder_data_path(&refs_path.join("logs").join("master"))?;
    let master_sha_path = refs_path.join("heads").join("master");
    let master_sha = std::fs::read(&master_sha_path)?;
    let mut reader = get_file_reader(folder_data_path);
    let fd = FolderData::new(&mut reader, &master_sha)?;
    Ok(fd.new_head_sha1)
}

pub fn get_file_reader(filename: PathBuf) -> BufReader<File> {
    let file = match File::open(&filename) {
        Ok(f) => f,
        Err(err) => panic!(
            "Could not open file {}: {}",
            filename.as_path().to_str().unwrap(),
            err
        ),
    };
    BufReader::new(file)
}

pub fn get_master_keys(path: &str, computer: &str) -> Result<Vec<Vec<u8>>> {
    let enc_path = Path::new(path).join(computer).join("encryptionv3.dat");
    let mut reader = get_file_reader(enc_path);
    let password = rpassword::prompt_password("Enter encryption password: ")?;
    let enc_data = object_encryption::EncryptionDat::new(&mut reader, &password)?;
    Ok(enc_data.master_keys)
}
