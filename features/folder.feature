Feature: detect folders

  Scenario: wants folder, folder exists
    Given a folder "node_modules"
    When running "has folder node_modules"
    Then it succeeds

  Scenario: wants folder, folder does not exist
    When running "has folder node_modules"
    Then it does not succeed

  Scenario: wants no folder, folder does exist
    Given a folder "node_modules"
    When running "has no folder node_modules"
    Then it does not succeed

  Scenario: wants no folder, folder does not exist
    When running "has no folder node_modules"
    Then it succeeds
