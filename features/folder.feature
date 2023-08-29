Feature: detect folders

  Scenario Outline: search for an exact match
    Given a folder "<FOLDER>"
    When running "<QUERY>"
    Then it signals <RESULT>
    And it prints nothing

    Examples:
      | DESCRIPTION                  | QUERY                   | FOLDER             | RESULT   |
      | matching folder              | has folder node_modules | node_modules       | match    |
      | matching folder in subfolder | has folder node_modules | tools/node_modules | no match |
      | partially matching folder    | has folder node         | node_modules       | no_match |

  Scenario Outline: search using a simple glob
    Given a folder "<FOLDER>"
    When running: "<QUERY>"
    Then it signals <RESULT>
    And it prints nothing

    Examples:
      | DESCRIPTION                    | QUERY            | FOLDER             | RESULT   |
      | matching folder in current dir | has folder node* | node_modules       | match    |
      | matching file in subfolder     | has folder node* | tools/node_modules | no match |
