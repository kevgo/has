Feature: detect files

  Scenario: the given file exists
    Given a file "package.json"
    When running "has file package.json"
    Then it succeeds
