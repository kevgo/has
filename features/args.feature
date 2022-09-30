Feature: Check CLI arguments

  Scenario: no arguments
    When running "has"
    Then it prints:
      """
      No target provided

      Usage: has [no] <target> <name>

      Targets define which type of object to check for:
      - branch (a local Git branch)
      - file
      - folder

      Name is the name of the object to check for.

      The "no" argument checks for absence of the given object.
      """

  Scenario: no name
    When running "has file"
    Then it prints:
      """
      No name provided

      Usage: has [no] <target> <name>

      Targets define which type of object to check for:
      - branch (a local Git branch)
      - file
      - folder

      Name is the name of the object to check for.

      The "no" argument checks for absence of the given object.
      """

  Scenario: duplicate name
    When running "has file foo bar"
    Then it prints:
      """
      Too many arguments

      Usage: has [no] <target> <name>

      Targets define which type of object to check for:
      - branch (a local Git branch)
      - file
      - folder

      Name is the name of the object to check for.

      The "no" argument checks for absence of the given object.
      """
