Feature: detect files by name

  Rule: searching by filename finds the given files

    Scenario: as expected, a file with the given name exists
      Given a file "package.json"
      When running:
        """
        has file package.json
        """
      Then it succeeds
      And it prints nothing

    Scenario: unexpectedly, no file with the given name exists
      When running:
        """
        has file package.json
        """
      Then it fails
      And it prints nothing

    Scenario: unexpectedly, a file with the given name exists
      Given a file "package.json"
      When running:
        """
        has no file package.json
        """
      Then it fails
      And it prints nothing

    Scenario: as expected, no file with the given name exists
      When running:
        """
        has no file package.json
        """
      Then it succeeds
      And it prints nothing

  Rule: searching by simple glob finds files only in the current directory

    Scenario: as expected, a file matching the given glob exists
      Given a file "package.json"
      When running:
        """
        has file *.json
        """
      Then it succeeds
      And it prints nothing

    Scenario: unexpectedly, no file matching the given glob exists
      When running:
        """
        has file *.json
        """
      Then it fails
      And it prints nothing

    Scenario: as expected, no file matching the given glob exists
      Given a file "package.json"
      When running:
        """
        has no file *.json
        """
      Then it fails
      And it prints nothing

    Scenario: as expected, no file matching the given glob exists
      When running:
        """
        has no file *.json
        """
      Then it succeeds
      And it prints nothing

    Scenario: a file matching the given simple glob exists but in a subfolder
      Given a file "alpha/beta/package.json"
      When running:
        """
        has file *.json
        """
      Then it fails
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

  Rule: allows only one file glob

    Scenario: multiple file globs given
      When running:
        """
        has file foo bar
        """
      Then it fails
      And the output starts with:
        """
        ERROR: too many arguments
        """
