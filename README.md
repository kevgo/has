# has

[![CI](https://github.com/kevgo/has/actions/workflows/ci.yml/badge.svg)](https://github.com/kevgo/has/actions/workflows/ci.yml)

`has` is a higher-level alternative for the traditional Unix `test` tool. It
allows querying a wide variety of properties of the local filesystem, source
code management systems, codebases, and build systems. One of its intended use
cases is within automation tooling like [mrt](https://github.com/kevgo/mrt).

The general usage is:

```
has [no] <condition>
```

`has` indicates the result of the check through its exit code: `0` means
success, any non-zero exit code indicates the given condition is not met. The
optional `no` argument inverts the condition, i.e. checks for absence of the
condition.

### files

Check whether a file exists ([examples](features/file-name.feature)):

```
has [no] file <glob>
```

Check whether file content includes the given text
([examples](features/file-content.feature)):

```
has [no] file <glob> --containing <text>
```

Check whether file content matches the given regex
([examples](features/file-content-regex.feature)):

```
has [no] file <glob> --matching <regex>
```

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

Check whether the Git repo contains commits of a particular author
([examples](features/git-author.feature)):

```
has [no] git-commits-by-author <name or email>
```

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
