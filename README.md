# has

[![CI](https://github.com/kevgo/has/actions/workflows/ci.yml/badge.svg)](https://github.com/kevgo/has/actions/workflows/ci.yml)

`has` is a modern replacement for the traditional `test` tool. It allows
querying properties of the local filesystem and Git repositories and indicates
success through its exit code.

### files and folders

Check file existence:

```
has [no] file <glob>
has [no] folder <glob>
```

Check file content:

```
has [no] file <glob> --containing <text>   # check file content
has [no] file <glob> --matching <regex>    # check file content
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
