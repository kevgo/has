Feature: detect unpushed changes

  Background:
    Given my code is managed by Git
    And my Git repo has a remote
    And my Git workspace is on the "feature" branch

  Scenario: wants unpushed changes, has them
    Given a local commit
    When running "has uncommitted-changes"
    Then it succeeds

  Scenario: wants unpushed changes, has none
    When running "has uncommitted-changes"
    Then it fails

  Scenario: wants no unpushed changes, has some
    Given a file "uncommitted.txt"
    When running "has no uncommitted-changes"
    Then it fails

  Scenario: wants no unpushed changes, has none
    When running "has no uncommitted-changes"
    Then it succeeds
