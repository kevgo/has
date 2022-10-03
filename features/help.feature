Feature: help screen

  Scenario: asking for help
    When running "has help"
    Then it prints:
      """

      Usage: has [no] <target> <name>

      Query files and folders:
      > has [no] file <file name>
      > has [no] folder <folder name>

      Query Git repositories:
      > has [no] branch <branch name>
      > has [no] uncommitted-changes
      > has [no] unpushed-changes

      Query command output
      > has [no] empty-output <command> [args...]

      The "no" argument checks for absence of the given object.
      """
