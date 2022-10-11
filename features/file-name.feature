Feature: detect files by name

  Rule: filename is required

    Scenario: no name provided
      When running:
        """
        has file
        """
      Then it prints:
        """
        ERROR: no name provided

        Usage: has [no] <target> <name>

        Query files and folders:
        > has [no] file <glob>                       # matches if a file matching the given glob exists
        > has [no] file <glob> --containing <text>   # matches if a file matching the given glob contains the given text
        > has [no] file <glob> --matching <regex>    # matches if a file matching the given glob contains the given text
        > has [no] folder <glob>                     # matches if a directory matching the given glob exists

        Query Git repositories:
        > has [no] branch <branch name>           # matches if the given branch exists
        > has [no] active-branch <branch name>    # matches if the given branch is checked out
        > has [no] inactive-branch <branch name>  # matches if the given branch exists but is not checked out
        > has [no] uncommitted-changes            # matches if the workspace contains uncommitted changes
        > has [no] unpushed-commits               # matches if the current branch contains commits that it's remote branch has not

        Query command output
        > has [no] empty-output <command> [args...]  # runs the given command and matches if it produces no output

        The optional "no" argument inverts the given condition.
        """

    Scenario: duplicate name argument
      When running:
        """
        has file foo bar
        """
      Then it prints:
        """
        ERROR: too many arguments

        Usage: has [no] <target> <name>

        Query files and folders:
        > has [no] file <glob>                       # matches if a file matching the given glob exists
        > has [no] file <glob> --containing <text>   # matches if a file matching the given glob contains the given text
        > has [no] file <glob> --matching <regex>    # matches if a file matching the given glob contains the given text
        > has [no] folder <glob>                     # matches if a directory matching the given glob exists

        Query Git repositories:
        > has [no] branch <branch name>           # matches if the given branch exists
        > has [no] active-branch <branch name>    # matches if the given branch is checked out
        > has [no] inactive-branch <branch name>  # matches if the given branch exists but is not checked out
        > has [no] uncommitted-changes            # matches if the workspace contains uncommitted changes
        > has [no] unpushed-commits               # matches if the current branch contains commits that it's remote branch has not

        Query command output
        > has [no] empty-output <command> [args...]  # runs the given command and matches if it produces no output

        The optional "no" argument inverts the given condition.
        """

  Rule: searching by filename finds the given files

    Scenario: wants file, file exists
      Given a file "package.json"
      When running:
        """
        has file package.json
        """
      Then it succeeds

    Scenario: wants file, file does not exist
      When running:
        """
        has file package.json
        """
      Then it fails

    Scenario: wants no file, file does exist
      Given a file "package.json"
      When running:
        """
        has no file package.json
        """
      Then it fails

    Scenario: wants no file, file does not exist
      When running:
        """
        has no file package.json
        """
      Then it succeeds
