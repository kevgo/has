# has

[![CI](https://github.com/kevgo/has/actions/workflows/ci.yml/badge.svg)](https://github.com/kevgo/has/actions/workflows/ci.yml)

`has` is a modern replacement for the traditional `test` tool. It allows
querying properties of the local filesystem and Git repositories and indicates
success through its exit code.

### query files and folders

```
has [no] file <glob>                       # matches if a file matching the given glob exists
has [no] file <glob> --containing <text>   # matches if a file matching the given glob contains the given text
has [no] folder <glob> --matching <regex>  # matches if a file matching the given glob contains text matching the given regex
```

### query Git repositories

```
has [no] branch <branch name>           # matches if the given branch exists
has [no] active-branch <branch name>    # matches if the given branch is checked out
has [no] inactive-branch <branch name>  # matches if the given branch exists but is not checked out
has [no] uncommitted-changes
has [no] unpushed-commits
```

### query command output

```
has [no] empty-output <command> [args...]  # runs the given command and matches if it produces no output
```
