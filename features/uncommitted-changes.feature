Feature: detect uncommitted changes

  Scenario: wants uncommitted changes, has them
    Given a Git branch "feature"
    And a file "uncommitted.txt"
    When running "has uncommitted-changes"
    Then it succeeds

  Scenario: wants uncommitted changes, has none
    Given a Git branch "feature"
    When running "has uncommitted-changes"
    Then it fails
