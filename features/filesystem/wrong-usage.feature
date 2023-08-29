Feature: display guidance when used wrong

  Scenario Outline: wrong arguments
    When running "<QUERY>"
    Then it fails
    And the output starts with "<MESSAGE>"

    Examples:
      | QUERY              | MESSAGE                   |
      | has file           | ERROR: no name provided   |
      | has file one two   | ERROR: too many arguments |
      | has folder         | ERROR: no name provided   |
      | has folder one two | ERROR: too many arguments |
