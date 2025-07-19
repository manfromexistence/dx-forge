//! # The Chronicle
//!
//! This module manages the sandboxed Git repository for `dx`.
//! It provides a safe interface to initialize, stage, and commit changes
//! made by the `dx` tool, without interfering with the user's own Git history.
//!
//! ---
//!
//! # DX Chronicle CLI Blueprint
//!
//! This document outlines the future CLI interface for interacting with the Chronicle.
//! The commands will be implemented in TypeScript and will call the underlying Rust engine.
//!
//! ## `dx chronicle status`
//! Shows a summary of the latest changes recorded by the forge.
//!
//! ### Output Example:
//! ```text
//! Chronicle Status:
//!   Green Branch:  3 (latest)
//!   Yellow Branch: 2 (latest)
//!   Red Branch:    1 (latest)
//! ```
//!
//! ## `dx chronicle log <branch>`
//! Displays the commit history for a specific branch.
//!
//! ### Usage Example:
//! `dx chronicle log green`
//!
//! ### Output Example:
//! ```text
//! History for 'green' branch:
//!   ID: 3 | Forged: /path/to/NewComponent.tsx
//!   ID: 2 | Forged: /path/to/AnotherComponent.tsx
//!   ID: 1 | Forged: /path/to/FirstComponent.tsx
//! ```
//!
//! ## `dx chronicle revert <branch> <id>`
//! Reverts the user's codebase to the state before a specific change was made.
//!
//! ### Usage Example:
//! `dx chronicle revert green 2`
//!
//! ### Action:
//! 1. Finds the change associated with the ID on the specified branch.
//! 2. Finds its parent commit.
//! 3. Compares the file trees of the two commits to see what changed.
//! 4. Reverts the change in the user's working directory.
//! 5. Creates a new commit in the Chronicle to record the revert action.
//!
//! ---

use anyhow::{anyhow, Result};
use chrono::Local;
use gix::actor::Signature;
use gix::prelude::*; // Import the prelude for easy access to extension traits
use std::path::Path;

// The name of the directory where our sandboxed git lives.
const CHRONICLE_DIR: &str = ".dx";

/// Ensures the Chronicle repository exists, initializing it if necessary.
pub fn initialize() -> Result<gix::Repository> {
    let chronicle_path = Path::new(CHRONICLE_DIR);

    if let Ok(repo) = gix::open(chronicle_path) {
        println!("Chronicle: Found existing repository.");
        return Ok(repo);
    }

    println!("Chronicle: Initializing new repository at '{}'...", chronicle_path.display());

    // Use `gix::init_bare` for explicit bare repository creation.
    let repo = gix::init_bare(chronicle_path)?;

    println!("Chronicle: Initialization complete.");
    Ok(repo)
}

/// Commits a file change to a specific branch in the Chronicle.
pub fn record_change(repo: &mut gix::Repository, file_path: &Path, branch_name: &str) -> Result<()> {
    println!("Chronicle: Staging '{}' for '{}' branch.", file_path.display(), branch_name);

    // --- 1. Find the Parent Commit and its Tree ---
    let branch_ref_name_str = format!("refs/heads/{}", branch_name);
    let branch_ref_name = gix::refs::FullName::try_from(branch_ref_name_str.as_str())?;
    
    let parent_commit_id: Option<gix::hash::ObjectId> = repo
        .find_reference(&branch_ref_name)
        .ok()
        .and_then(|mut r| r.peel_to_id_in_place().ok())
        .map(|id| id.detach());

    let parent_tree = if let Some(id) = &parent_commit_id {
        let commit = repo.find_object(*id)?.try_into_commit()?;
        Some(commit.tree()?)
    } else {
        None
    };

    // --- 2. Create the New Tree ---
    let mut new_tree = match parent_tree {
        Some(tree) => {
            let tree_ref = tree.decode()?;
            gix::objs::Tree {
                entries: tree_ref.entries.into_iter().map(|e| gix::objs::tree::Entry {
                    mode: e.mode,
                    filename: e.filename.to_owned(),
                    oid: e.oid,
                }).collect(),
            }
        }
        None => gix::objs::Tree::empty(),
    };

    let blob_id = repo.write_blob(std::fs::read(file_path)?)?;
    let file_name = gix::bstr::BString::from(file_path.file_name().unwrap().to_str().unwrap());
    
    new_tree.entries.push(gix::objs::tree::Entry {
        mode: gix::objs::tree::EntryKind::Blob.into(),
        oid: blob_id.into(),
        filename: file_name,
    });
    
    let new_tree_id = repo.write_object(&new_tree)?;

    // --- 3. Create the Revert ID and Detailed Commit Message ---
    let now = Local::now();
    let commit_title = now.format("%Y-%m-%d-(%I:%M:%S%P)").to_string().to_lowercase();

    let revert_id = if branch_name == "green" {
        let daily_commit_count = get_daily_commit_count(repo, parent_commit_id.clone())?;
        format!("{}-{}", now.format("%Y-%m-%d"), daily_commit_count + 1)
    } else {
        let sequential_count = get_sequential_commit_count(repo, parent_commit_id.clone())?;
        format!("{}", sequential_count + 1)
    };
    
    let commit_body = format!(
        "forged: {}\nbranch: {}\nid: {}",
        file_path.display(),
        branch_name,
        revert_id
    );
    let full_commit_message = format!("{}\n\n{}", commit_title, commit_body);
    
    // --- 4. Create and Write the Commit Object ---
    let time = gix::date::Time::now_local_or_utc();
    let author = Signature {
        name: "dx-forge".into(),
        email: "forge@dx.local".into(),
        time,
    };
    
    let parents: Vec<gix::hash::ObjectId> = parent_commit_id.into_iter().collect();

    let commit = gix::objs::Commit {
        message: full_commit_message.into(),
        tree: new_tree_id.into(),
        author: author.clone(),
        committer: author,
        encoding: None,
        parents: parents.into(),
        extra_headers: Default::default(),
    };

    let commit_id: gix::hash::ObjectId = repo.write_object(commit)?.into();

    // --- 5. Update the Branch Reference ---
    repo.reference(
        branch_ref_name.as_ref(),
        commit_id,
        gix::refs::transaction::PreviousValue::Any,
        "dx: commit",
    )?;

    println!("Chronicle: Commit successful. ID: {} -> {}", branch_name, revert_id);
    Ok(())
}

/// Counts how many commits were made today on the 'green' branch.
fn get_daily_commit_count(repo: &gix::Repository, head: Option<gix::hash::ObjectId>) -> Result<u32> {
    let Some(head) = head else { return Ok(0) };

    let today_start = Local::now().date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp();
    let mut count = 0;

    for commit_info in head.ancestors(repo) {
        let commit_info = commit_info?;
        let commit = repo.find_object(commit_info.id)?.try_into_commit()?;
        let time = commit.committer.time.seconds;
        if time < today_start {
            break;
        }
        count += 1;
    }
    Ok(count)
}

/// Counts the total number of commits on a branch to create the next sequential ID.
fn get_sequential_commit_count(repo: &gix::Repository, head: Option<gix::hash::ObjectId>) -> Result<u32