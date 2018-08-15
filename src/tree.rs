use {GitDirectory, DirectoryEntry, TreeEntryExt};
use std::cmp::Ordering;
use git2::Tree;

/// A set of extension methods for the `Tree` type in `git2`.
pub trait TreeExt {
    /// Get a list of all the named entries at this level in the tree.
    fn entry_names(&self) -> Vec<String>;

    /// Get a higher level, pre-sorted view of this level in the tree analogous to
    /// a traditional filesystem, separated into directories and files.
    fn dir_listing(&self) -> GitDirectory;
}

impl<'repo> TreeExt for Tree<'repo> {
    fn entry_names(&self) -> Vec<String> {
        self.iter()
            .filter_map(|n|
                if let Some(s) = n.name() {
                    Some(s.to_owned())
                } else {
                    None
                })
            .collect()
    }

    fn dir_listing(&self) -> GitDirectory {
        let mut dirs: Vec<DirectoryEntry> = vec![];
        let mut files: Vec<DirectoryEntry> = vec![];

        let names = self.entry_names();

        names.into_iter().for_each(|name| {
            if let Some(node) = self.get_name(&name) {
                let entry = DirectoryEntry {
                    name,
                    node,
                };

                if entry.node.is_dir() {
                    dirs.push(entry);
                } else {
                    files.push(entry);
                }
            }
        });

        let sort_alpha: fn(&DirectoryEntry, &DirectoryEntry) -> Ordering =
            |a, b| a.name.cmp(&b.name);

        dirs.sort_by(sort_alpha);
        files.sort_by(sort_alpha);

        GitDirectory {
            dirs,
            files,
        }
    }
}