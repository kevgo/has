Feature: detect files by name and content

  Background:
    Given a file "package.json" with content:
      """
      {
        "dependencies": {
          "prettier": "^1.2.3"
        }
      }
      """

  Rule: the "containing" clause additionally matches the file content verbatim

    Scenario Outline: queries using the "containing" clause
      When running "<QUERY>"
      Then it signals <RESULT>
      And it prints nothing

      Examples:
        | DESCRIPTION                           | QUERY                                     | RESULT   |
        | matching filename and content         | has file package.json --containing ^1.2.3 | match    |
        | matching name but mismatching content | has file package.json --containing zonk   | no match |

  Rule: the "matching" clause additionally matches the file content via regex

    Scenario Outline: queries using the "matching" clause
      When running "<QUERY>"
      Then it signals <RESULT>
      And it prints nothing

      Examples:
        | DESCRIPTION                             | QUERY                                            | RESULT   |
        | matching filename and content           | has file package.json --matching prettier.*1.2.3 | match    |
        | matchging the glob and matching content | has file *.json --matching prettier.*1.2.3       | match    |
        | matching name but mismatching content   | has file package.json --matching prettier.*1.2.4 | no match |
