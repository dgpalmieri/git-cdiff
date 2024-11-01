# Under development
This project should be considered Pre-Alpha, and probably contains more bugs than
actually working code. Any and all functionality described below may or may not be
implemented.


# git-cdiff

Show `git diff`s for different types of
[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).

# Examples

```bash
gcdiff HEAD~5..HEAD~15 --include="docs","chore"
```

Show a diff over all commits between `HEAD~5` and `HEAD~15` that are prefixed with
`docs*:` or `chore*:`. This includes tags like `docs(readability):` and `chore(dependencies):`
in the diff.

```bash
gcdiff HEAD~10 --exclude="test"
```

Show a diff over all commits between `HEAD` and `HEAD~10` that are __not__ prefixed with
`test*:`. This excludes tags like `test(remote):` and `test(ci/cd):` from the diff.

```bash
gcdiff HEAD~3 --exclude="test" --iterative
```

Opens three panes in the git pager where page one is `git diff HEAD~3..HEAD~2`, page two
is `git diff HEAD~2..HEAD~1`, etc. Excludes all commits whose message starts with
`test*:`
