Feature: display guidance when used wrong

  Scenario Outline: wrong arguments
    When running "<QUERY>"
    Then it fails
    And the output starts with "<MESSAGE>"

    Examples:
      | QUERY              | MESSAGE                       |
      | has command-output | ERROR: missing command to run |
