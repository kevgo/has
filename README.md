# has

[![CI](https://github.com/kevgo/has/actions/workflows/ci.yml/badge.svg)](https://github.com/kevgo/has/actions/workflows/ci.yml)

`has` is a modern replacement for the traditional `test` tool. It allows
querying properties of the local filesystem and Git repositories. One of its
intended use cases is within [mrt](https://github.com/kevgo/mrt).

The general usage is:

```
has [no] <condition>
```

`has` indicates the result of the check through its exit code, `0` meaning
success. The optional `no` argument inverts the condition, i.e. checks for
absence of the condition.

### files

Check whether a file exists ([examples](features/file-name.feature)):

```
has [no] file <glob>
```

Check whether a file includes the given text
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

### Git commits

Check whether a Git repo contains uncommitted changes
([examples](features/uncommitted-changes.feature)):

```
has [no] uncommitted-changes
```

Check for Git commits not on the remote branch
([examples](features/unpushed-commits.feature)):

```
has [no] unpushed-commits
```

### shell commands

Runs the given command and checks that it outputs anything
([examples](features/command-output.feature)):

```
has [no] command-output <command> [args...]
```
