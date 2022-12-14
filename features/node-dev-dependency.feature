Feature: detect Node dev-depenndencies

  Scenario: has it, wants it
    Given a file "package.json" with content:
      """
      {
        "name": "foo",
        "devDependencies": {
          "alpha": "1.0.0.",
          "license-checker": "2.0.0"
        }
      }
      """
    When running:
      """
      has nodejs-dev-dependency license-checker
      """
    Then it prints nothing
    And it succeeds

  Scenario: has it, doesn't wants it
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
      has no nodejs-dev-dependency beta
      """
    Then it prints nothing
    And it fails

  Scenario: doesn't have it, wants it
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
      has nodejs-dev-dependency beta
      """
    Then it prints nothing
    And it fails

  Scenario: doesn't have it, doesn't want it
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
      has no nodejs-dev-dependency beta
      """
    Then it prints nothing
    And it succeeds

  Scenario: no Node codebase
    When running:
      """
      has nodejs-dev-dependency alpha
      """
    Then it prints nothing
    And it fails
