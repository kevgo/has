Feature: detect files by name and content

  Background:
    Given a file "package.json" with content:
      """
      {
        "dependencies": {
          "prettier": "1.2.3"
        }
      }
      """

  Rule: if a "containing" clause is given, the file name and content must match

    Scenario: file with matching name and content
      When running:
        """
        has file package.json --containing '"prettier":'
        """
      Then it succeeds
      And it prints nothing

    Scenario: file with matching name but mismatching content
      When running:
        """
        has file package.json --containing "zonk"
        """
      Then it fails
      And it prints nothing
