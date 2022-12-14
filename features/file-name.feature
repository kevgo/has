Feature: detect files by name

  Rule: searching by filename finds the given files

    Scenario: wants file, file exists
      Given a file "package.json"
      When running:
        """
        has file package.json
        """
      Then it succeeds
      And it prints nothing

    Scenario: wants file, file does not exist
      When running:
        """
        has file package.json
        """
      Then it fails
      And it prints nothing

    Scenario: wants no file, file does exist
      Given a file "package.json"
      When running:
        """
        has no file package.json
        """
      Then it fails
      And it prints nothing

    Scenario: wants no file, file does not exist
      When running:
        """
        has no file package.json
        """
      Then it succeeds
      And it prints nothing

  Rule: searching by simple glob finds files in the current directory

    Scenario: wants file, file exists
      Given a file "package.json"
      When running:
        """
        has file *.json
        """
      Then it succeeds
      And it prints nothing

    Scenario: wants file, file does not exist
      When running:
        """
        has file *.json
        """
      Then it fails
      And it prints nothing

    Scenario: wants no file, file does exist
      Given a file "package.json"
      When running:
        """
        has no file *.json
        """
      Then it fails
      And it prints nothing

    Scenario: wants no file, file does not exist
      When running:
        """
        has no file *.json
        """
      Then it succeeds
      And it prints nothing

  Rule: searching by nested glob finds nested files

    Scenario: wants file, file exists
      Given a file "alpha/beta/package.json"
      When running:
        """
        has file **/*.json
        """
      Then it succeeds
      And it prints nothing

    Scenario: wants file, file does not exist
      When running:
        """
        has file **/package.*
        """
      Then it fails
      And it prints nothing

    Scenario: wants no file, file does exist
      Given a file "alpha/beta/package.json"
      When running:
        """
        has no file **/*.json
        """
      Then it fails
      And it prints nothing

    Scenario: wants no file, file does not exist
      When running:
        """
        has no file **/package.*
        """
      Then it succeeds
      And it prints nothing

  Rule: searching by nested glob finds files in the current directory

    Scenario: wants file, file exists
      Given a file "package.json"
      When running:
        """
        has file **/*.json
        """
      Then it succeeds
      And it prints nothing

    Scenario: wants file, file does not exist
      When running:
        """
        has file **/package.*
        """
      Then it fails
      And it prints nothing

    Scenario: wants no file, file does exist
      Given a file "package.json"
      When running:
        """
        has no file **/*.json
        """
      Then it fails
      And it prints nothing

    Scenario: wants no file, file does not exist
      When running:
        """
        has no file **/package.*
        """
      Then it succeeds
      And it prints nothing

  Rule: filename is required

    Scenario: no name provided
      When running:
        """
        has file
        """
      Then it fails
      And the output starts with:
        """
        ERROR: no name provided
        """

    Scenario: duplicate name argument
      When running:
        """
        has file foo bar
        """
      Then it fails
      And the output starts with:
        """
        ERROR: too many arguments
        """
