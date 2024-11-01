use git2::{Commit, Diff, DiffOptions, Error, Oid, Repository};
use std::path::Path;

pub struct GitDiffWrapper {
    repo: Repository,
}

impl GitDiffWrapper {
    /// Create a new GitDiffWrapper for the given repository path
    pub fn new<P: AsRef<Path>>(repo_path: P) -> Result<Self, Error> {
        let repo = Repository::open(repo_path)?;
        Ok(Self { repo })
    }

    /// Get the diff between two commits
    pub fn get_diff(&self, commit1: &str, commit2: Option<&str>) -> Result<String, Error> {
        let c1 = self.resolve_commit(commit1)?;
        let c2 = match commit2 {
            Some(commit) => Some(self.resolve_commit(commit)?),
            None => None,
        };

        let t1 = c1.tree()?;
        let t2 = match c2 {
            Some(commit) => commit.tree()?,
            None => self.repo.head()?.peel_to_tree()?,
        };

        let mut diff_opts = DiffOptions::new();
        let diff = self
            .repo
            .diff_tree_to_tree(Some(&t1), Some(&t2), Some(&mut diff_opts))?;

        let mut diff_string = String::new();
        diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
            use std::str;
            if let Ok(str) = str::from_utf8(line.content()) {
                diff_string.push_str(str);
                true
            } else {
                false
            }
        })?;

        Ok(diff_string)
    }

    /// Get iterative diffs for the last n commits
    pub fn get_iterative_diffs(&self, num_commits: u32) -> Result<Vec<String>, Error> {
        let mut diffs = Vec::new();
        for i in (1..=num_commits).rev() {
            let older = format!("HEAD~{}", i);
            let newer = format!("HEAD~{}", i - 1);
            let diff = self.get_diff(&older, Some(&newer))?;
            diffs.push(diff);
        }
        Ok(diffs)
    }

    /// Find files that were modified in both commits
    pub fn find_common_files(&self, commit1: &str, commit2: &str) -> Result<Vec<String>, Error> {
        let c1 = self.resolve_commit(commit1)?;
        let c2 = self.resolve_commit(commit2)?;

        let files1 = self.get_modified_files(&c1)?;
        let files2 = self.get_modified_files(&c2)?;

        Ok(files1.intersection(&files2).cloned().collect())
    }

    // Helper methods
    fn resolve_commit(&self, spec: &str) -> Result<Commit, Error> {
        let obj = self.repo.revparse_single(spec)?;
        obj.peel_to_commit()
    }

    fn get_modified_files(
        &self,
        commit: &Commit,
    ) -> Result<std::collections::HashSet<String>, Error> {
        let parent = commit.parent(0)?;
        let t1 = parent.tree()?;
        let t2 = commit.tree()?;

        let mut diff_opts = DiffOptions::new();
        let diff = self
            .repo
            .diff_tree_to_tree(Some(&t1), Some(&t2), Some(&mut diff_opts))?;

        let mut files = std::collections::HashSet::new();
        diff.foreach(
            &mut |delta, _| {
                if let Some(path) = delta.new_file().path() {
                    if let Some(path_str) = path.to_str() {
                        files.insert(path_str.to_string());
                    }
                }
                true
            },
            None,
            None,
            None,
        )?;

        Ok(files)
    }
}

fn main() -> Result<(), Error> {
    // Example usage
    let git = GitDiffWrapper::new(".")?;

    // Get simple diff
    let diff = git.get_diff("HEAD~1", Some("HEAD"))?;
    println!("Diff between HEAD~1 and HEAD:\n{}", diff);

    // Get iterative diffs
    let diffs = git.get_iterative_diffs(3)?;
    for (i, diff) in diffs.iter().enumerate() {
        println!("\nDiff {}:\n{}", i + 1, diff);
    }

    // Find common files
    let common_files = git.find_common_files("HEAD~2", "HEAD")?;
    println!("\nFiles modified in both commits: {:?}", common_files);

    Ok(())
}
