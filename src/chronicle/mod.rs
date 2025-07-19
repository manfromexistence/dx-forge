//! # The Chronicle
//!
//! This module manages the sandboxed Git repository for `dx`.
//! It provides a safe interface to initialize, stage, and commit changes
//! made by the `dx` tool, without interfering with the user's own Git history.

use anyhow::Result;
use chrono::Local;
use gix::actor::Signature;
use std::path::Path;

// The name of the directory where our sandboxed git lives.
const CHRONICLE_DIR: &str = ".dx/chronicle";

/// Ensures the Chronicle repository exists, initializing it if necessary.
pub fn initialize() -> Result<gix::Repository> {
    let chronicle_path = Path::new(CHRONICLE_DIR);

    if let Ok(repo) = gix::open(chronicle_path) {
        println!("Chronicle: Found existing repository.");
        return Ok(repo);
    }

    println!("Chronicle: Initializing new repository at '{}'...", chronicle_path.display());

    if let Some(parent) = chronicle_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }

    // Use `gix::init_bare` for explicit bare repository creation.
    let repo = gix::init_bare(chronicle_path)?;

    println!("Chronicle: Initialization complete.");
    Ok(repo)
}

/// Commits a newly generated file to the Chronicle.
pub fn commit_file(repo: &mut gix::Repository, file_to_add: &Path, branch_name: &str) -> Result<()> {
    println!("Chronicle: Committing '{}' to '{}' branch.", file_to_add.display(), branch_name);

    // 1. Create a blob from the file content.
    let blob_id = repo.write_blob(std::fs::read(file_to_add)?)?.into();

    // 2. Create a tree for this commit.
    let mut tree = gix::objs::Tree::empty();
    let file_name = gix::bstr::BString::from(file_to_add.file_name().unwrap().to_str().unwrap());
    tree.entries.push(gix::objs::tree::Entry {
        mode: gix::objs::tree::EntryKind::Blob.into(),
        oid: blob_id,
        filename: file_name,
    });

    // 3. Write the tree object to the database.
    let tree_id = repo.write_object(&tree)?.into();
    
    // 4. Create the human-readable commit message.
    let now = Local::now();
    let message = format!("forged: {}\n\nbranch: {}", now.format("%Y-%m-%d %H:%M:%S"), branch_name);
    
    // 5. Create the author and committer signature.
    let time = gix::date::Time::now_local_or_utc();
    let author = Signature {
        name: "dx-forge".into(),
        email: "forge@dx.local".into(),
        time,
    };
    
    // 6. Get the parent commit if HEAD exists.
    let parents: Vec<gix::hash::ObjectId> = match repo.head_id() {
        Ok(head_id) => vec![head_id.into()],
        Err(_) => vec![], // No HEAD exists, so this is the first commit.
    };

    // 7. Create the commit object.
    let commit = gix::objs::Commit {
        message: message.into(),
        tree: tree_id,
        author: author.clone(),
        committer: author,
        encoding: None,
        parents: parents.into(),
        extra_headers: Default::default(),
    };

    // 8. Write the commit object to the database.
    // *** THE FIX: Add an explicit type annotation for the commit ID. ***
    let commit_id: gix::hash::ObjectId = repo.write_object(&commit)?.into();

    // 9. Update HEAD to point to our new commit.
    repo.reference("HEAD", commit_id, gix::refs::transaction::PreviousValue::Any, "dx: commit")?;

    println!("Chronicle: Commit successful. Hash: {}", commit_id);
    Ok(())
}
