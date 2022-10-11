# has

[![CI](https://github.com/kevgo/has/actions/workflows/ci.yml/badge.svg)](https://github.com/kevgo/has/actions/workflows/ci.yml)

`has` is a modern replacement for the traditional `test` tool. It allows
querying properties of the local filesystem and Git repositories and indicates
success through its exit code.

### files and folders

Check existence of file using globs ([example](features/file-name.feature)):

```
has [no] file <glob>
```

Check existence of folder using glob:

```
has [no] folder <glob>
```

Check for part of file content:

```
has [no] file <glob> --containing <text>
```

Check for part of file content using regex:

```
has [no] file <glob> --matching <regex>
```

### Git repositories

Check Git branches:

```
has [no] branch <branch name>           # matches if the given branch exists
has [no] active-branch <branch name>    # matches if the given branch is checked out
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
