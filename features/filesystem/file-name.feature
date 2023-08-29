Feature: detect files by name

  Scenario Outline: search for an exact filename
    Given a file "<FILE>"
    When running "<QUERY>"
    Then it signals <RESULT>
    And it prints nothing

    Examples:
      | DESCRIPTION                                            | QUERY                    | FILE                    | RESULT   |
      | matching file in current dir                           | has file package.json    | package.json            | match    |
      | negation                                               | has no file package.json | package.json            | no match |
      | matching file in subfolder                             | has file package.json    | alpha/beta/package.json | no match |
      | partially matching filename                            | has file package.js      | package.json            | no match |
      | simple glob with matching file in current dir          | has file *.json          | package.json            | match    |
      | simple glob with matching file in subfolder            | has file *.json          | alpha/beta/package.json | no match |
      | simple glob with partial match                         | has file *.js            | package.json            | no match |
      | double-asterisk glob with matching file in current dir | has file **/*.json       | package.json            | match    |
      | double-asterisk glob with matching file in subfolder   | has file **/*.json       | alpha/beta/package.json | match    |
      | double-asterisk glob with partial match                | has file **/*.js         | package.json            | no match |
