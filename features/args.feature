Feature: Check CLI arguments

  Scenario: no arguments
    When running "has"
    Then it prints:
      """
      ERROR: no target provided

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

  Scenario: no name
    When running "has file"
    Then it prints:
      """
      ERROR: no name provided

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

  Scenario: duplicate name
    When running "has file foo bar"
    Then it prints:
      """
      ERROR: too many arguments

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
