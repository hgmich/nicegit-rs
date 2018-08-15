use git2::TreeEntry;

/// A friendly interface to a 'directory' in git, which is a snapshot of a node in the
/// working tree at a specific commit which would be mapped to a file system directory
/// by a Git client, which is a flat, single level of a subtree. The directory and file nodes
/// are split into two separate lists and sorted in UTF-8 codepoint order.
pub struct GitDirectory<'repo> {
    /// The list of directories in this directory, sorted in UTF-8 codepoint order.
    pub dirs: Vec<DirectoryEntry<'repo>>,

    /// The list of files in this directory, sorted in UTF-8 codepoint order.
    pub files: Vec<DirectoryEntry<'repo>>,
}

/// A simple struct for a `TreeEntry` prepackaged with it's name for easier use.
pub struct DirectoryEntry<'repo> {
    /// The file/directory name, without any relative or absolute paths.
    pub name: String,

    /// The `TreeEntry` with this name.
    pub node: TreeEntry<'repo>,
}
