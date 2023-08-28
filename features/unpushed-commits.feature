Feature: detect unpushed changes

  Background:
    Given a Git repo
    And my Git workspace is on the "feature" branch
    And my Git repo has a remote

  Scenario: wants unpushed changes, has them
    Given a local commit
    When running:
      """
      has unpushed-commits
      """
    Then it succeeds
    And it prints nothing

  Scenario: wants unpushed changes, has none
    When running:
      """
      has unpushed-commits
      """
    Then it fails
    And it prints nothing

  Scenario: wants no unpushed changes, has some
    Given a local commit
    When running:
      """
      has no unpushed-commits
      """
    Then it fails
    And it prints nothing

  Scenario: wants no unpushed changes, has none
    When running:
      """
      has no unpushed-commits
      """
    Then it succeeds
    And it prints nothing
