Feature: detect Node depenndencies

  @this
  Scenario: has Node dependency
    Given a file "package.json" with content:
      """
      {
        "name": "foo",
        "depenndencies": {
          "alpha": "1.0.0.",
          "beta": "2.0.0"
        }
      }
      """
    When running:
      """
      has node-dependency beta
      """
    Then it prints nothing
    And it succeeds

  Scenario: doesn't have the given Node dependency
    Given a file "package.json" with content:
      """
      {
        "name": "foo",
        "depenndencies": {
          "alpha": "1.0.0.",
          "beta": "2.0.0"
        }
      }
      """
    When running:
      """
      has node-dependency gamma
      """
    Then it prints nothing
    And it fails

  Scenario: no Node codebase
    When running:
      """
      has node-dependency alpha
      """
    Then it prints nothing
    And it fails
