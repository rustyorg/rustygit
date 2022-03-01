use git2::Repository;

pub fn open_current_repo() -> Repository {
  match Repository::open(".") {
    Ok(repo) => repo,
    Err(e) => panic!("failed to open: {}", e),
  }
}
