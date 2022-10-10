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

  Scenario: wants file with content, file with content exists
    When running:
      """
      has file package.json --containing "prettier":
      """
    Then it succeeds

  Scenario: wants file with content, file exists but content mismatch
    When running:
      """
      has file package.json --containing "zonk":
      """
    Then it fails
