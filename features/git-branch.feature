Feature: detect Git branches

  Scenario: wants branch, branch exists
    Given a Git branch "feature"
    When running "has branch feature"
    Then it succeeds

  Scenario: wants branch, branch does not exist
    When running "has branch feature"
    Then it does not succeed

  Scenario: wants no branch, branch does exist
    Given a Git branch "feature"
    When running "has no branch feature"
    Then it does not succeed

  Scenario: wants no branch, branch does not exist
    When running "has no branch feature"
    Then it succeeds
