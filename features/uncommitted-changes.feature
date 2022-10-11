Feature: detect uncommitted changes

  Background:
    Given my code is managed by Git
    And my Git workspace is on the "feature" branch

  Scenario: wants uncommitted changes, has them
    Given a file "uncommitted.txt"
    When running:
      """
      has uncommitted-changes
      """
    Then it succeeds
    And it prints nothing

  Scenario: wants uncommitted changes, has none
    When running:
      """
      has uncommitted-changes
      """
    Then it fails
    And it prints nothing

  Scenario: wants no uncommitted changes, has some
    Given a file "uncommitted.txt"
    When running:
      """
      has no uncommitted-changes
      """
    Then it fails
    And it prints nothing

  Scenario: wants no uncommitted changes, has none
    When running:
      """
      has no uncommitted-changes
      """
    Then it succeeds
    And it prints nothing
