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

- [`file [glob]`](features/file-name.feature): a file with the given name exists
- [`file <glob> --containing <text>`](features/file-content.feature): a file with given name and content exists
- [`file <glob> --matching <regex>`](features/file-content-regex.feature): a file with given name and content matching the given regex exists

### folders

Check whether a folder exists ([examples](features/folder.feature)):

```
has [no] folder <glob>
```

### Git branches

Check whether a Git branch exists ([examples](features/git-branch.feature)):

```
has [no] branch <branch name>
```

Check the currently checked out Git branch
([examples](features/git-branch-active.feature)):

```
has [no] active-branch <branch name>
```

Check that a branch exists but is not checked out
([examples](features/git-branch-inactive.feature)):

```
has [no] inactive-branch <branch name>
```

### uncommitted changes

Check whether a Git workspace contains uncommitted changes
([examples](features/uncommitted-changes.feature)):

```
has [no] uncommitted-changes
```

### Git commits

Check for Git commits that haven't been pushed to the remote branch
([examples](features/unpushed-commits.feature)):

```
has [no] unpushed-commits
```

### shell commands

Runs the given command and checks that it outputs something
([examples](features/command-output.feature)):

```
has [no] command-output <command> [args...]
```

### Makefile

Check whether a given [Make](https://www.gnu.org/software/make) target exists
([examples](features/make-target.feature)):

```
has [no] make-target <name>
```

### Node.JS

Check whether the given Node.JS codebase contains a production dependency
([examples](features/node-dependency.feature)):

```
has [no] nodejs-dependency <name>
```

Check whether the given Node.JS codebase contains a development dependency
([examples](features/node-dependency.feature)):

```
has [no] nodejs-dev-dependency <name>
```
