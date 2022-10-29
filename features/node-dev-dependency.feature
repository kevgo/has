@this
Feature: detect Node dev-depenndencies

  Scenario: has Node dev-dependency
    Given a file "package.json" with content:
      """
      {
        "name": "foo",
        "devDependencies": {
          "alpha": "1.0.0.",
          "beta": "2.0.0"
        }
      }
      """
    When running:
      """
      has node-dev-dependency beta
      """
    Then it prints nothing
    And it succeeds

  Scenario: doesn't have the given Node dependency
    Given a file "package.json" with content:
      """
      {
        "name": "foo",
        "devDependencies": {
          "alpha": "1.0.0."
        }
      }
      """
    When running:
      """
      has node-dev-dependency beta
      """
    Then it prints nothing
    And it fails

  Scenario: no Node codebase
    When running:
      """
      has node-dev-dependency alpha
      """
    Then it prints nothing
    And it fails
