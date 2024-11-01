use git_cdiff::{GitDiffWrapper, Result};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).map(|s| s.as_str()).unwrap_or(".");

    let git = GitDiffWrapper::new(path)?;

    // Show diff between HEAD and HEAD~1
    let diff = git.get_diff("HEAD~1", Some("HEAD"))?;
    println!("Latest commit diff:\n{}", diff);

    Ok(())
}
