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

        The optional "no" argument inverts the condition.

        Check files by name and contents:
        > has [no] file <glob>
        > has [no] file <glob> --containing <text>
        > has [no] file <glob> --matching <regex>

        Check folders:
        > has [no] folder <glob>

        Check Git branches:
        > has [no] branch <branch name>
        > has [no] active-branch <branch name>
        > has [no] inactive-branch <branch name>

        Check uncommitted changes in a Git workspace:
        > has [no] uncommitted-changes

        Check Git commits:
        > has [no] unpushed-commits

        Check command output:
        > has [no] command-output <command> [args...]  # runs the given command and matches if it produces no output

        Check Node.JS dependencies:
        > has [no] nodejs-dependency <name>
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

        The optional "no" argument inverts the condition.

        Check files by name and contents:
        > has [no] file <glob>
        > has [no] file <glob> --containing <text>
        > has [no] file <glob> --matching <regex>

        Check folders:
        > has [no] folder <glob>

        Check Git branches:
        > has [no] branch <branch name>
        > has [no] active-branch <branch name>
        > has [no] inactive-branch <branch name>

        Check uncommitted changes in a Git workspace:
        > has [no] uncommitted-changes

        Check Git commits:
        > has [no] unpushed-commits

        Check command output:
        > has [no] command-output <command> [args...]  # runs the given command and matches if it produces no output

        Check Node.JS dependencies:
        > has [no] nodejs-dependency <name>
        """
