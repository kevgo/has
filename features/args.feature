Feature: Check CLI arguments

  Scenario: no arguments
    When running "has"
    Then it prints:
      """
      No target provided
      """

  Scenario: no name
    When running "has file"
    Then it prints:
      """
      No name provided
      """

  Scenario: duplicate name
    When running "has file foo bar"
    Then it prints:
      """
      Too many arguments
      """
