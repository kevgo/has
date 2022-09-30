Feature: help screen

  Scenario: asking for help
    When running "has help"
    Then it prints:
      """

      Usage: has [no] <target> <name>

      Targets define which type of object to check for:
      - branch (a local Git branch)
      - file
      - folder
      - help (print help)

      Name is the name of the object to check for.

      The "no" argument checks for absence of the given object.
      """
