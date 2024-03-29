Feature: display guidance when used wrong

  Scenario Outline: wrong arguments
    When running "<QUERY>"
    Then it fails
    And the output starts with "<MESSAGE>"

    Examples:
      | QUERY                   | MESSAGE                    |
      | has make-target         | ERROR: missing Make target |
      | has make-target one two | ERROR: too many arguments  |

  Scenario: no Makefile
    When running:
      """
      has make-target foo
      """
    Then it prints nothing
    And it fails
