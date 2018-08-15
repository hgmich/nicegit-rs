use git2::{TreeEntry, ObjectType};

/// A set of extension methods for the `Tree` type in `git2`.
pub trait TreeEntryExt {
    /// Returns whether this entry in the tree represents a subtree, which would be a subdirectory
    /// on disk.
    fn is_dir(&self) -> bool;
}

impl<'repo> TreeEntryExt for TreeEntry<'repo> {
    fn is_dir(&self) -> bool {

        match self.kind() {
            Some(ObjectType::Tree) => true,
            _ => false
        }
    }
}
