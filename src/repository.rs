use git2::{Branch, BranchType, Reference, Repository};
use std::collections::HashMap;

/// These paths are used to emulate CLI-style 'do what I mean' refname resolution
/// in ref_dwim_local.
const LOCAL_DWIM_SEARCH_PATH: [&str; 4] = [
    "",
    "refs/",
    "refs/tags/",
    "refs/heads/",
];

/// A set of extension methods for the `Repository` type in git2.
pub trait RepositoryExt {
    /// Gets a list of all branch names for the repository, with `branch_type` letting you filter by
    /// local, known remote, or all known branches.
    fn branch_names(&self, branch_type: Option<BranchType>) -> Vec<String>;

    /// Gets a mapping of all local branches to their actual branch objects.
    fn local_branches(&self) -> HashMap<String, Branch>;

    /// Do a "do what I mean" lookup on a refname - the 'fuzzy' sort you're used to from the CLI.
    /// In order of priority, this becomes:
    ///   * Symbolic names and commit hashes
    ///   * Named refs
    ///   * Tags by exact name
    ///   * The HEAD of a named branch
    /// You need to exactly match one of the above, or this search will fail.
    fn ref_dwim_local(&self, name: &str) -> Option<Reference>;

    /// Get the name of the currently checked out branch, if one is checked out.
    fn current_branch_name(&self) -> Option<String>;
}

impl RepositoryExt for Repository {
    fn branch_names(&self, branch_type: Option<BranchType>) -> Vec<String> {
        let avail_branches = self.branches(branch_type).unwrap();

        avail_branches
            .filter_map(|b| b.ok())
            .map(|b| b.0)
            .filter_map(|b|
                match b.name() {
                    Ok(name) => if let Some(name) = name {
                        Some(name.to_owned())
                    } else {
                        None
                    }
                    _ => None
                }
            )
            .collect()
    }

    fn local_branches(&self) -> HashMap<String, Branch> {
        let branches = self.branch_names(Some(BranchType::Local));
        let mut branch_map: HashMap<String, Branch> = HashMap::new();

        branches.into_iter().for_each(|branch_name| {
            if let Ok(branch) = self.find_branch(&branch_name, BranchType::Local) {
                branch_map.insert(branch_name, branch);
            }
        });

        branch_map
    }

    fn ref_dwim_local(&self, name: &str) -> Option<Reference> {
        for path in &LOCAL_DWIM_SEARCH_PATH {
            let ref_name = path.to_string() + name;
            if let Ok(refid) = self.find_reference(&ref_name) {
                return Some(refid);
            }
        }

        None
    }

    fn current_branch_name(&self) -> Option<String> {
        self
            .find_reference("HEAD")
            .ok()
            .iter()
            .filter_map(|r| r.resolve().ok())
            .map(|r| Box::new(Branch::wrap(r)))
            .next()
            .iter()
            .filter_map(|b| b.name().ok())
            .filter_map(|n| n)
            .map(|n| n.to_owned())
            .next()
    }
}
