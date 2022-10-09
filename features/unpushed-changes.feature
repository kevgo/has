Feature: detect unpushed changes

  Background:
    Given my code is managed by Git
    And my Git workspace is on the "feature" branch
    And my Git repo has a remote

  Scenario: wants unpushed changes, has them
    Given a local commit
    When running:
      """
      has unpushed-changes
      """
    Then it succeeds

  Scenario: wants unpushed changes, has none
    When running:
      """
      has unpushed-changes
      """
    Then it fails

  Scenario: wants no unpushed changes, has some
    Given a local commit
    When running:
      """
      has no unpushed-changes
      """
    Then it fails

  Scenario: wants no unpushed changes, has none
    When running:
      """
      has no unpushed-changes
      """
    Then it succeeds
