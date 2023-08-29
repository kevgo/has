Feature: display guidance when used wrong

  Scenario Outline: wrong arguments
    When running "<QUERY>"
    Then it fails
    And the output starts with "<MESSAGE>"

    Examples:
      | QUERY                             | MESSAGE                                                               |
      | has nodejs-dependency             | ERROR: please provide the name of the Node dependency to look for     |
      | has nodejs-dependency one two     | ERROR: too many arguments                                             |
      | has nodejs-dev-dependency         | ERROR: please provide the name of the Node dev-dependency to look for |
      | has nodejs-dev-dependency one two | ERROR: too many arguments                                             |

  Scenario Outline: no Node codebase
    When running "<QUERY>"
    Then it fails
    And it prints nothing

    Examples:
      | QUERY                           |
      | has nodejs-dependency alpha     |
      | has nodejs-dev-dependency alpha |
