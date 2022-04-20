/**
 * This module contains helper methods for types in the git2 library
 * in order to provide a higher-level or simpler API where useful.
 */
use std::path::Path;

use anyhow::{anyhow, Result};
use git2::{Repository, Status, StatusEntry};

pub mod branch;

pub fn open_current_repo() -> Repository {
  match Repository::open(".") {
    Ok(repo) => repo,
    Err(e) => panic!("failed to open: {}", e),
  }
}

pub trait StatusExt {
  fn is_staged(&self) -> bool;
}

impl StatusExt for Status {
  fn is_staged(&self) -> bool {
    self.is_index_new()
      || self.is_index_modified()
      || self.is_index_deleted()
      || self.is_index_renamed()
      || self.is_index_typechange()
  }
}

pub trait GetPath {
  fn get_path(&self) -> Result<&Path>;
}

impl GetPath for StatusEntry<'_> {
  fn get_path(&self) -> Result<&Path> {
    let path_str = self.path().ok_or_else(|| anyhow!("Invalid path"))?;
    Ok(Path::new(path_str))
  }
}
//
// pub fn get_current_branch_name(repo: &Repository) -> Result<String, Error> {
//   let head = match repo.head() {
//     Ok(head) => Some(head),
//     Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => None,
//     Err(e) => return Err(e),
//   };
//
//   match head.as_ref().and_then(|h| h.shorthand()) {
//     None => Ok("".to_string()),
//     Some(branch) => Ok(branch.to_string()),
//   }
// }
//
// pub fn get_repo_head(repo: &Repository) -> Option<Reference> {
//   match repo.head() {
//     Ok(head) => Some(head),
//     Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => None,
//     Err(_e) => None,
//   }
// }
//
// pub fn get_remote_branch(repo: &Repository, branch_name: &str) -> Option<git2::Branch<'_>> {
//   match repo.find_branch(branch_name, BranchType::Remote) {
//     Ok(branch) => Some(branch),
//     Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => None,
//     Err(e) => None,
//   }
// }
//
// pub fn get_local_branch(repo: &Repository, branch_name: &str) -> git2::Branch<'_> {
//   repo.find_branch(branch_name, BranchType::Local).unwrap()
// }
//
// pub fn get_current_branch(repo: &Repository) -> Result<Branch, Error> {
//   let head = get_repo_head(repo);
//
//   let current_branch_name = head.as_ref().and_then(|h| h.shorthand()).unwrap_or("");
//   let current_branch_remote_name = format!("origin/{}", current_branch_name);
//
//   let branch_local = get_local_branch(repo, current_branch_name);
//   let branch_remote = get_remote_branch(repo, current_branch_remote_name.as_str());
//
//   let mut ahead_behind = Some((0, 0));
//   let local_oid = branch_local
//
//   if branch_remote.is_some() {
//     let remote_oid = branch_remote
//       .as_ref()
//       .unwrap()
//       .get()
//       .peel(ObjectType::Commit)
//       .unwrap()
//       .id();
//
//
//   };
//
//   Ok(Branch {
//     name: name.to_string(),
//     branch_remote,
//     branch_local,
//     ahead_behind,
//   })
// }
