Feature: help screen

  Rule: providing no arguments shows usage instructions

    Scenario: no arguments
      When running:
        """
        has
        """
      Then it prints:
        """
        ERROR: no target provided

        Usage: has [no] <target> <name>

        The optional "no" argument inverts the given condition.

        QUERY FILES AND FOLDERS:

        # matches if a file matching the given glob exists
        has [no] file <glob>

        # matches if a file matching the given glob contains the given text
        has [no] file <glob> --containing <text>

        # matches if a file matching the given glob contains the given text
        has [no] file <glob> --matching <regex>

        # matches if a directory matching the given glob exists
        has [no] folder <glob>

        QUERY GIT REPOSITORIES:

        # matches if the given branch exists
        has [no] branch <branch name>

        # matches if the given branch is checked out
        has [no] active-branch <branch name>

        # matches if the given branch exists but is not checked out
        has [no] inactive-branch <branch name>

        # matches if the workspace contains uncommitted changes
        has [no] uncommitted-changes

        # matches if the current branch contains commits that it's remote branch has not
        has [no] unpushed-commits

        QUERY COMMAND OUTPUT

        # runs the given command and matches if it produces no output
        has [no] empty-output <command> [args...]
        """

  Rule: the "help" command displays usage instructions

    Scenario: asking for help
      When running:
        """
        has help
        """
      Then it prints:
        """

        Usage: has [no] <target> <name>

        Query files and folders:
        > has [no] file <glob>
        > has [no] file <glob> --containing <text>   # check file content
        > has [no] file <glob> --matching <regex>    # check file content
        > has [no] folder <glob>

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
