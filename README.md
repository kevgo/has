# has

[![CI](https://github.com/kevgo/has/actions/workflows/ci.yml/badge.svg)](https://github.com/kevgo/has/actions/workflows/ci.yml)

`has` is a modern replacement for the traditional `test` tool. It allows
querying properties of the local filesystem and Git repositories and indicates
success through its exit code.

The general usage is:

```
has [no] <condition>
```

The optional `no` argument inverts the condition, i.e. checks for absence of the
condition.

### files

Check whether a file exists ([example](features/file-name.feature)):

```
has [no] file <glob>
```

Check whether a file includes the given text:

```
has [no] file <glob> --containing <text>
```

Check whether file content matches the given regex:

```
has [no] file <glob> --matching <regex>
```

### folders

Check whether a folder exists ([example](features/folder.feature)):

```
has [no] folder <glob>
```

### Git branches

Check whether a Git branch exists:

```
has [no] branch <branch name>
```

Check the currently checked out Git branch:

```
has [no] active-branch <branch name>
```

Check that a branch exists but is not checked out:

```
has [no] inactive-branch <branch name>
```

### Git commits

Check whether a Git repo contains uncommitted changes:

```
has [no] uncommitted-changes
```

Check whether a Git branch contains commits that its remote branch doesn't have:

```
has [no] unpushed-commits
```

### shell commands

Runs the given command and checks that it doesn't output anything:

```
has [no] empty-output <command> [args...]
```
