Feature: detect inactive Git branches

  Background:
    Given a Git repo

  Scenario: wants inactive branch, branch is inactive
    Given my Git workspace has a branch "feature"
    And my Git workspace is on the "main" branch
    When running:
      """
      has git-branch-inactive feature
      """
    Then it succeeds
    And it prints nothing

  Scenario: wants inactive branch, branch is active
    Given my Git workspace is on the "feature" branch
    When running:
      """
      has git-branch-inactive feature
      """
    Then it fails
    And it prints nothing

  Scenario: wants inactive branch, branch does not exist
    When running:
      """
      has git-branch-inactive feature
      """
    Then it fails
    And it prints nothing
