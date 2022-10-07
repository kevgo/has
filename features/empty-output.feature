Feature: detect empty command output

  Scenario: empty output
    When running "has empty-output echo"
    Then it succeeds

  Scenario: output containing newlines
    When running "has empty-output printf \n"
    Then it succeeds

  Scenario: non-empty output
    When running "has empty-output echo Hello world!"
    Then it fails
