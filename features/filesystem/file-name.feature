Feature: detect files by name

  Rule: the given file name must match

    Scenario: file with exact name
      Given a file "package.json"
      When running:
        """
        has file package.json
        """
      Then it succeeds
      And it prints nothing

    Scenario: no file with matching name
      When running:
        """
        has file package.json
        """
      Then it fails
      And it prints nothing

  Rule: if negated, there must be no file with a matching name

    Scenario: no file with matching name
      When running:
        """
        has no file package.json
        """
      Then it succeeds
      And it prints nothing

    Scenario: a file with matching name exists
      Given a file "package.json"
      When running:
        """
        has no file package.json
        """
      Then it fails
      And it prints nothing

  Rule: simple globs match only files in the current directory

    Scenario: file with matching name
      Given a file "package.json"
      When running:
        """
        has file *.json
        """
      Then it succeeds
      And it prints nothing

    Scenario: no file with matching names
      When running:
        """
        has file *.json
        """
      Then it fails
      And it prints nothing

    Scenario:
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
