Feature: detect files

  Rule: finds files by name

    Scenario: wants file, file exists
      Given a file "package.json"
      When running "has file package.json"
      Then it succeeds

    Scenario: wants file, file does not exist
      When running "has file package.json"
      Then it fails

    Scenario: wants no file, file does exist
      Given a file "package.json"
      When running "has no file package.json"
      Then it fails

    Scenario: wants no file, file does not exist
      When running "has no file package.json"
      Then it succeeds

  Rule: finds files by glob

    Scenario: wants file, file exists
      Given a file "package.json"
      When running "has file *.json"
      Then it succeeds

    Scenario: wants file, file does not exist
      When running "has file *.json"
      Then it fails

    Scenario: wants no file, file does exist
      Given a file "package.json"
      When running "has no file *.json"
      Then it fails

    Scenario: wants no file, file does not exist
      When running "has no file *.json"
      Then it succeeds
