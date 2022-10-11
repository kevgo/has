Feature: help screen

  Rule: the "help" command displays usage instructions

    Scenario: asking for help
      When running:
        """
        has help
        """
      Then it succeeds
      And it prints:
        """

        Usage: has [no] <condition>

        The optional "no" argument inverts the given condition.

        Check for the existence of files by name and contents:
        > has [no] file <glob>
        > has [no] file <glob> --containing <text>
        > has [no] file <glob> --matching <regex>

        Check for the existence of folders:
        > has [no] folder <glob>

        Check for the existence and condition of Git branches:
        > has [no] branch <branch name>
        > has [no] active-branch <branch name>
        > has [no] inactive-branch <branch name>

        Check for the existence of changes that haven't been committed yet:
        > has [no] uncommitted-changes

        Check for the existence of commits that don't exist on the tracking branch:
        > has [no] unpushed-commits

        Check whether the given command produces no output:
        > has [no] command-output <command> [args...]  # runs the given command and matches if it produces no output
        """

  Rule: providing no arguments shows usage instructions

    Scenario: no arguments
      When running:
        """
        has
        """
      Then it fails
      And it prints:
        """
        ERROR: no target provided

        Usage: has [no] <condition>

        The optional "no" argument inverts the given condition.

        Check for the existence of files by name and contents:
        > has [no] file <glob>
        > has [no] file <glob> --containing <text>
        > has [no] file <glob> --matching <regex>

        Check for the existence of folders:
        > has [no] folder <glob>

        Check for the existence and condition of Git branches:
        > has [no] branch <branch name>
        > has [no] active-branch <branch name>
        > has [no] inactive-branch <branch name>

        Check for the existence of changes that haven't been committed yet:
        > has [no] uncommitted-changes

        Check for the existence of commits that don't exist on the tracking branch:
        > has [no] unpushed-commits

        Check whether the given command produces no output:
        > has [no] command-output <command> [args...]  # runs the given command and matches if it produces no output
        """
