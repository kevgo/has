# has

[![CI](https://github.com/kevgo/has/actions/workflows/ci.yml/badge.svg)](https://github.com/kevgo/has/actions/workflows/ci.yml)

`has` is a modern replacement for the traditional `test` tool. It allows
querying properties of the local filesystem and Git repositories and indicates
success through its exit code.

### query files and folders

```
has [no] file <file name>
has [no] folder <folder name>
```

### query Git repositories

```
has [no] branch <branch name>
has [no] uncommitted-changes
has [no] unpushed-changes
```

### query command output

```
has [no] empty-output <command> [args...]
```
