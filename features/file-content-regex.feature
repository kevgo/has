Feature: searching for a file  via regex

  Background:
    Given a file "package.json" with content:
      """
      {
        "dependencies": {
          "prettier": "1.2.3"
        }
      }
      """

  Rule: if a "matching" clause is given, the file name must match the given glob and the content must match the given regex

    Scenario: file with matching name and content
      When running:
        """
      has file package.json --matching prettier.*1.2.3
        """
      Then it succeeds
      And it prints nothing

    Scenario: file with name matchging the glob and matching content
      When running:
        """
      has file *.json --matching prettier.*1.2.3
        """
      Then it succeeds
      And it prints nothing

    Scenario: file with matching name but mismatching content
      When running:
        """
      has file package.json --matching prettier.*1.2.4
        """
      Then it fails
      And it prints nothing
