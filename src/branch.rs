use git2::{Branch, Commit};

/// A set of extension methods for the `Branch` type in git2.
pub trait BranchExt {
    /// Gets the `Commit` at the tip of this `Branch`.
    fn tip_commit(&self) -> Option<Commit>;
}

impl<'repo> BranchExt for Branch<'repo> {
    fn tip_commit(&self) -> Option<Commit> {
        self.get().peel_to_commit().ok()
    }
}