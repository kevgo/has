Feature: display guidance when used wrong

  Scenario Outline: wrong arguments
    When running "<QUERY>"
    Then it fails
    And the output starts with "<MESSAGE>"

    Examples:
      | QUERY                           | MESSAGE                   |
      | has git-branch                  | ERROR: no name provided   |
      | has git-branch one two          | ERROR: too many arguments |
      | has git-branch-active           | ERROR: no name provided   |
      | has git-branch-active one two   | ERROR: too many arguments |
      | has git-branch-inactive         | ERROR: no name provided   |
      | has git-branch-inactive one two | ERROR: too many arguments |
      | has git-changes-uncommitted one | ERROR: too many arguments |
      | has git-commits-unpushed one    | ERROR: too many arguments |
