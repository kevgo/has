# has

[![CI](https://github.com/kevgo/has/actions/workflows/ci.yml/badge.svg)](https://github.com/kevgo/has/actions/workflows/ci.yml)

`has` is the programmer's version of the Unix `test` tool. It allows querying a
wider variety of properties from the local computer than just the file system.
Examples are the configuration of the codebases in the local folder as well as
the status and configuration of source code management and build systems for
them. The general usage is:

```
has [no] <condition>
```

`has` indicates the result of the check through its exit code: `0` means
success, any non-zero exit code indicates the given condition is not met. The
optional `no` argument inverts the condition, i.e. checks for absence of the
condition.

### conditions

- [`has file <glob>`](features/file-name.feature): file with a matching name
- [`has file <glob> --containing <text>`](features/file-name-and-content.feature):
  file with matching name and content
- [`has file <glob> --matching <regex>`](features/file-name-and-content.feature):
  file with matching name and content matching the given regex
- [`has folder <name>`](features/folder.feature): folder with the given name
- [`has git-branch <branch name>`](features/git-branch.feature): a Git branch
  with the given name
- [`has git-branch-active <branch name>`](features/git-branch-active.feature):
  the currently checked out Git branch has the given name
- [`has git-branch-inactive <branch name>`](features/git-branch-inactive.feature):
  a Git branch with the given name that is not checked out
- [`has git-changes-uncommitted`](features/git-changes-uncommitted.feature): the
  local Git workspace contains uncommitted changes
- [`has git-commits-by-author`](features/git-commits-by-author.feature): the
  local Git workspace contains commits by the given person
- [`has git-commits-unpushed`](features/git-commits-unpushed.feature): unpushed
  commits in the local Git workspace
- [`has command-output <command> [args...]`](features/command-output.feature):
  the given shell command prints something to STDOUT
- [`has make-target <name>`](features/make-target.feature): the
  [Makefile](https://www.gnu.org/software/make) contains the given target
- [`has nodejs-dependency <name>`](features/node-dependency.feature): a Node.JS
  codebase using the given production dependency
- [`has nodejs-dev-dependency <name>`](features/node-dependency.feature): a
  Node.JS codebase using the given development dependency
