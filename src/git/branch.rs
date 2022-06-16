use git2::{BranchType, ErrorCode, ObjectType, Oid, Reference, Repository};

pub struct Branch<'a> {
  pub name: Option<String>,
  pub remote: Option<git2::Branch<'a>>,
  pub local: Option<git2::Branch<'a>>,
  pub ahead_behind: (usize, usize),
  pub repo: &'a Repository,
}

impl<'a> Branch<'a> {
  pub fn new(repo: &'a Repository) -> Branch {
    Branch {
      name: Some("main".to_string()),
      remote: None,
      local: None,
      ahead_behind: (0, 0),
      repo,
    }
  }

  pub fn initialize(&mut self) {
    self.name = self.get_current_branch_name();
    if self.name.is_some() {
      let name = self.name.clone().unwrap();
      self.local = self.get_branch(&name, BranchType::Local);
      self.remote = self.get_branch(&name, BranchType::Remote);
      self.ahead_behind = self.get_ahead_behind(&self.local, &self.remote);
    }
  }

  pub fn get_ahead_behind(
    &self,
    local_branch: &Option<git2::Branch>,
    upstream_branch: &Option<git2::Branch>,
  ) -> (usize, usize) {
    if local_branch.is_none() || upstream_branch.is_none() {
      return (0, 0);
    }

    self
      .repo
      .graph_ahead_behind(
        self.get_branch_oid(local_branch).unwrap(),
        self.get_branch_oid(upstream_branch).unwrap(),
      )
      .unwrap_or((0, 0))
  }

  pub fn get_current_branch_name(&self) -> Option<String> {
    let head = self.get_repo_head();

    if let Some(branch) = head {
      if branch.shorthand().is_some() {
        let name = branch.shorthand().unwrap().to_string();
        return Some(name);
      }
      None
    } else {
      None
    }
  }

  pub fn get_repo_head(&self) -> Option<Reference> {
    match self.repo.head() {
      Ok(head) => Some(head),
      Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => None,
      Err(_e) => None,
    }
  }

  pub fn get_branch(
    &self,
    branch_name: &String,
    branch_type: BranchType,
  ) -> Option<git2::Branch<'a>> {
    let name = match branch_type {
      BranchType::Local => branch_name.clone(),
      BranchType::Remote => format!("origin/{}", branch_name),
    };

    match self.repo.find_branch(name.as_str(), branch_type) {
      Ok(branch) => Some(branch),
      Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => None,
      Err(_e) => None,
    }
  }

  pub fn get_branch_oid(&self, branch: &Option<git2::Branch>) -> Option<Oid> {
    if branch.is_none() {
      None
    } else {
      let id = branch
        .as_ref()
        .unwrap()
        .get()
        .peel(ObjectType::Commit)
        .unwrap()
        .id();
      Some(id)
    }
  }
}
