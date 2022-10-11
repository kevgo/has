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

### files and folders

Check that a file exists ([example](features/file-name.feature)):

```
has [no] file <glob>
```

Check that a folder exists ([example](features/folder.feature)):

```
has [no] folder <glob>
```

Check that a file includes the given text:

```
has [no] file <glob> --containing <text>
```

Check that file content matches the given regex:

```
has [no] file <glob> --matching <regex>
```

### Git repositories

Check whether a Git branch exists:

```
has [no] branch <branch name>           # matches if the given branch exists
```

Check the currently checked out Git branch:

```
has [no] active-branch <branch name>    # matches if the given branch is checked out
```

```
has [no] inactive-branch <branch name>  # matches if the given branch exists but is not checked out
```

Check Git commits:

```
has [no] uncommitted-changes
has [no] unpushed-commits
```

### shell commands

Runs the given command and checks that it doesn't output anything:

```
has [no] empty-output <command> [args...]
```
