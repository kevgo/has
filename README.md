# has

[![CI](https://github.com/kevgo/has/actions/workflows/ci.yml/badge.svg)](https://github.com/kevgo/has/actions/workflows/ci.yml)

`has` is the programmer's version of the Unix `test` tool. It
allows querying a wider variety of properties of the local computer system, source
code management systems, codebases and their configurations, and build systems.

The general usage is:

```
has [no] <condition>
```

`has` indicates the result of the check through its exit code: `0` means
success, any non-zero exit code indicates the given condition is not met. The
optional `no` argument inverts the condition, i.e. checks for absence of the
condition.

### conditions

- [`file <glob>`](features/file-name.feature): a file with the given name exists
- [`file <glob> --containing <text>`](features/file-content.feature): a file with the given name and content exists
- [`file <glob> --matching <regex>`](features/file-content-regex.feature): a file with given name and content matching the given regex exists
- [`folder <glob>`](features/folder.feature): a folder with the given name exists
- [`git-branch <branch name>`](features/git-branch.feature): a Git branch with the given name exists
- [`git-branch-active <branch name>`](features/git-branch-active.feature): the currently checked out Git branch has the given name
- [`git-branch-inactive <branch name>`](features/git-branch-inactive.feature): a Git with the given name exists but is not checked out
- [`git-changes-uncommitted`](features/git-changes-uncommitted.feature): the local Git workspace contains uncommitted changes
- [`git-commits-unpushed`](features/git-commits-unpushed.feature): Git contains local commits that haven't been pushed to the tracking branch
- [`command-output <command> [args...]`](features/command-output.feature): runs the given shell command and verifies that it prints something to STDOUT
- [`make-target <name>`](features/make-target.feature): a given [Make](https://www.gnu.org/software/make) target exists
- [`nodejs-dependency <name>`](features/node-dependency.feature): the Node.JS codebase in the current folder contains the given production dependency
- [`nodejs-dev-dependency <name>`](features/node-dependency.feature): the Node.JS codebase in the current folder contains the given development dependency
