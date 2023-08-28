# has

[![CI](https://github.com/kevgo/has/actions/workflows/ci.yml/badge.svg)](https://github.com/kevgo/has/actions/workflows/ci.yml)

`has` is the programmer's version of the Unix `test` tool. It allows querying a
wide variety of properties of the local computer system. Examples are the status
and configuration of source code management and build systems or the
configuration of the codebases in the local folder.

The general usage is:

```
has [no] <condition>
```

`has` indicates the result of the check through its exit code: `0` means
success, any non-zero exit code indicates the given condition is not met. The
optional `no` argument inverts the condition, i.e. checks for absence of the
condition.

### conditions

- [`has file <glob>`](features/filesystem/file-name.feature): file with a matching name
- [`has file <glob> --containing <text>`](features/filesystem/features/file-content.feature): file
  with matching name and content
- [`has file <glob> --matching <regex>`](features/filesystem/features/file-content-regex.feature): checks for a
  file with matching name and content matching the given regex
- [`has folder <glob>`](features/folder.feature): a folder with the given name
  exists
- [`has git-branch <branch name>`](features/git-branch.feature): a Git branch with
  the given name exists
- [`has git-branch-active <branch name>`](features/git-branch-active.feature): the
  currently checked out Git branch has the given name
- [`has git-branch-inactive <branch name>`](features/git-branch-inactive.feature): a
  Git with the given name exists but is not checked out
- [`has git-changes-uncommitted`](features/git-changes-uncommitted.feature): the
  local Git workspace contains uncommitted changes
- [`has git-commits-by-author`](features/git-commits-by-author.feature): the local
  Git workspace contains commits by the given person
- [`has git-commits-unpushed`](features/git-commits-unpushed.feature): Git contains
  local commits that haven't been pushed to the tracking branch
- [`has command-output <command> [args...]`](features/command-output.feature): the
  given shell command prints something to STDOUT
- [`has make-target <name>`](features/make-target.feature): a given
  [Make](https://www.gnu.org/software/make) target exists
- [`has nodejs-dependency <name>`](features/node-dependency.feature): the Node.JS
  codebase uses the the given production dependency
- [`has nodejs-dev-dependency <name>`](features/node-dependency.feature): the
  Node.JS codebase uses the given development dependency
