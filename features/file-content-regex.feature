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
        has file package.json --contains prettier"
      """
    Then it succeeds

  Scenario: wants file with content, file exists but content mismatch
    Given a file "package.json" with content:
      """
      {
        "dependencies": {
          "prettier": "1.2.3"
        }
      }
      """
    When running:
      """
        has file package.json --contains '"zonk":'"
      """
    Then it fails
