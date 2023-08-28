Feature: searching for content via regex

  Background:
    Given a file "package.json" with content:
      """
      {
        "dependencies": {
          "prettier": "1.2.3"
        }
      }
      """

  @this
  Scenario: wants file with content, file with content exists
    When running:
      """
      has file package.json --matching prettier.*1.2.3
      """
    Then it succeeds
    And it prints nothing

  Scenario: wants file with content, file exists but content mismatch
    When running:
      """
      has file package.json --matching prettier.*1.2.4"
      """
    Then it fails
    And it prints nothing
