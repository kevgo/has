Feature: detect inactive Git branches

  Background:
    Given my code is managed by Git

  Scenario: wants inactive branch, branch is active
    Given my Git workspace is on the "feature" branch
    When running:
      """
      has active-branch feature
      """
    Then it succeeds

  Scenario: wants inactive branch, branch is inactive
    Given my Git workspace has a branch "feature"
    And my Git workspace is on the "main" branch
    When running:
      """
      has active-branch feature
      """
    Then it fails

  Scenario: wants inactive branch, branch does not exist
    When running:
      """
      has active-branch feature
      """
    Then it fails
