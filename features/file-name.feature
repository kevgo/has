Feature: detect files by name

  Scenario Outline: search for an exact filename
    Given a file "<FILE>"
    When running: "<QUERY>"
    Then it signals <RESULT>
    And it prints nothing

    Examples:
      | DESCRIPTION                  | QUERY                    | FILE                    | RESULT   |
      | matching file in current dir | has file package.json    | package.json            | match    |
      | negation                     | has no file package.json | package.json            | no match |
      | matching file in subfolder   | has file package.json    | alpha/beta/package.json | no match |
      | partially matching filename  | has file package.js      | package.json            | no match |

  Scenario Outline: search using a simple glob
    Given a file "<FILE>"
    When running: "<QUERY>"
    Then it signals <RESULT>
    And it prints nothing

    Examples:
      | DESCRIPTION                  | QUERY           | FILE                    | RESULT   |
      | matching file in current dir | has file *.json | package.json            | match    |
      | matching file in subfolder   | has file *.json | alpha/beta/package.json | no match |
      | not an exact match           | has file *.js   | package.json            | no match |

  Scenario Outline: search using a double asterisk glob
    Given a file "<FILE>"
    When running: "<QUERY>"
    Then it signals <RESULT>
    And it prints nothing

    Examples:
      | DESCRIPTION                  | QUERY              | FILE                    | RESULT   |
      | matching file in current dir | has file **/*.json | package.json            | match    |
      | matching file in subfolder   | has file **/*.json | alpha/beta/package.json | match    |
      | not an exact match           | has file *.js      | package.json            | no match |

  Scenario Outline: wrong arguments
    When running "<QUERY>"
    Then it fails
    And the output starts with "<MESSAGE>"

    Examples:
      | QUERY            | MESSAGE                   |
      | has file         | ERROR: no name provided   |
      | has file one two | ERROR: too many arguments |
