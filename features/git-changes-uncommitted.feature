Feature: detect uncommitted changes

  Background:
    Given a Git repo
    And my Git workspace is on the "feature" branch

  Scenario: wants uncommitted changes, has them
    Given a file "uncommitted.txt"
    When running:
      """
      has git-changes-uncommitted
      """
    Then it succeeds
    And it prints nothing

  Scenario: wants uncommitted changes, has none
    When running:
      """
      has git-changes-uncommitted
      """
    Then it fails
    And it prints nothing

  Scenario: wants no uncommitted changes, has some
    Given a file "uncommitted.txt"
    When running:
      """
      has no git-changes-uncommitted
      """
    Then it fails
    And it prints nothing

  Scenario: wants no uncommitted changes, has none
    When running:
      """
      has no git-changes-uncommitted
      """
    Then it succeeds
    And it prints nothing
