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

        Check filesystem:
        > has [no] file <glob>
        > has [no] file <glob> --containing <text>
        > has [no] file <glob> --matching <regex>
        > has [no] folder <glob>

        Check Git:
        > has [no] git-branch <branch name>
        > has [no] git-branch-active <branch name>
        > has [no] git-branch-inactive <branch name>
        > has [no] git-changes-uncommitted
        > has [no] git-commits-unpushed

        Check shell commands:
        > has [no] command-output <command> [args...]  # runs the given command and matches if it produces no output

        Check Makefiles:
        > has [no] make-target <name>

        Check Node.JS codebases:
        > has [no] nodejs-dependency <name>
        > has [no] nodejs-dev-dependency <name>
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
        ERROR: no condition provided

        Usage: has [no] <condition>

        The optional "no" argument inverts the condition.

        Check filesystem:
        > has [no] file <glob>
        > has [no] file <glob> --containing <text>
        > has [no] file <glob> --matching <regex>
        > has [no] folder <glob>

        Check Git:
        > has [no] git-branch <branch name>
        > has [no] git-branch-active <branch name>
        > has [no] git-branch-inactive <branch name>
        > has [no] git-changes-uncommitted
        > has [no] git-commits-unpushed

        Check shell commands:
        > has [no] command-output <command> [args...]  # runs the given command and matches if it produces no output

        Check Makefiles:
        > has [no] make-target <name>

        Check Node.JS codebases:
        > has [no] nodejs-dependency <name>
        > has [no] nodejs-dev-dependency <name>
        """
