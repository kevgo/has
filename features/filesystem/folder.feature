Feature: detect folders

  Scenario Outline: search for an exact match
    Given a folder "<FOLDER>"
    When running "<QUERY>"
    Then it signals <RESULT>
    And it prints nothing

    Examples:
      | DESCRIPTION                  | QUERY                      | FOLDER             | RESULT   |
      | matching folder              | has folder node_modules    | node_modules       | match    |
      | negation                     | has no folder node_modules | other              | match    |
      | matching folder in subfolder | has folder node_modules    | tools/node_modules | no match |
      | partially matching folder    | has folder node            | node_modules       | no match |
