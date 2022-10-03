Feature: detect Git branches

  Background:
    Given my code is managed by Git

  Scenario: is on branch
    Given my Git workspace is on the branch "feature"
    When running "has branch feature"
    Then it succeeds

  @this
  Scenario: wants branch, branch exists
    Given my Git workspace has a branch "feature"
    And my Git workspace is on the branch "main"
    When running "has branch feature"
    Then it succeeds

  Scenario: wants branch, branch does not exist
    When running "has branch feature"
    Then it fails

  Scenario: wants no branch, is on branch
    Given my Git workspace has a branch "feature"
    When running "has no branch feature"
    Then it fails

  Scenario: wants no branch, branch does exist
    Given my Git workspace has a branch "feature"
    And my Git workspace is on the branch "main"
    When running "has no branch feature"
    Then it fails

  Scenario: wants no branch, branch does not exist
    When running "has no branch feature"
    Then it succeeds
