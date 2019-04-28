use std::io::Cursor;
use std::path::{Path, PathBuf};

use crate::error::Result;
use crate::recovery;
use crate::utils;

use arq::tree;

fn show_commit(commit: &tree::Commit) {
    println!(
        "   - author: {}, comment: {}, version: {}, location: {}",
        &commit.author, &commit.comment, &commit.version, &commit.folder_path
    );
    println!("   - failed files count: {}", &commit.failed_files.len());
    println!("   - has missing nodes: {}", &commit.has_missing_nodes);
    println!("   - is complete: {}", &commit.is_complete);
    println!("   - arq version: {}", &commit.arq_version);
    println!("   - tree sha1: {}", &commit.tree_sha1);
    println!("   - date: {}", &commit.creation_date);
    println!("   - tree compression: {:?}", &commit.tree_compression_type);
    if !commit.parent_commits.is_empty() {
        println!("   ::Parent commits::");
        for parent_commit in commit.parent_commits.keys() {
            println!("    - {}", parent_commit);
        }
    }
}

pub fn show(path: &str, computer: &str, folder: &str) -> Result<()> {
    println!("Tree\n----");
    println!("\nComputer: {}, Folder: {}\n", computer, folder);

    let trees_path = std::path::Path::new(path)
        .join(computer)
        .join("packsets")
        .join(format!("{}-trees", folder));
    let master_keys = utils::get_master_keys(&path, &computer)?;
    let arq_folder = utils::read_arq_folder(path, computer, folder, master_keys.clone())?;
    let head_sha = utils::find_latest_folder_sha(path, computer, folder)?;

    render_tree(
        Path::new(&arq_folder.local_path),
        &trees_path,
        &head_sha,
        &master_keys[0],
    )
}

fn render_tree(
    prefix: &std::path::Path,
    path: &std::path::PathBuf,
    sha: &str,
    master_key: &[u8],
) -> Result<()> {
    let data = recovery::restore_blob_with_sha(path, sha, master_key)?;
    let commit = tree::Commit::new(Cursor::new(data))?;
    //show_commit(&commit);

    let tree_blob = recovery::restore_blob_with_sha(path, &commit.tree_sha1, master_key)?;
    let tree = tree::Tree::new(&tree_blob, commit.tree_compression_type)?;
    render_internal_tree(prefix, &path, tree, master_key)?;
    Ok(())
}

fn render_internal_tree(
    prefix: &std::path::Path,
    path: &PathBuf,
    tr: tree::Tree,
    master_key: &[u8],
) -> Result<()> {
    for (k, v) in tr.nodes {
        if v.is_tree {
            if v.data_blob_keys.is_empty() {
                continue;
            }
            let data =
                recovery::restore_blob_with_sha(&path, &v.data_blob_keys[0].sha1, &master_key)?;
            let tree = tree::Tree::new(&data, v.data_compression_type)?;
            render_internal_tree(prefix.join(k).as_path(), &path, tree, &master_key)?;
        } else {
            println!("{}", prefix.join(k).as_os_str().to_str().unwrap());
        }
    }
    Ok(())
}
