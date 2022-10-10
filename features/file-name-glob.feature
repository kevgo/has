Feature: detect file by glob

  Rule: searching by simple glob

    Scenario: wants file, file exists
      Given a file "package.json"
      When running:
        """
        has file *.json
        """
      Then it succeeds

    Scenario: wants file, file does not exist
      When running:
        """
        has file *.json
        """
      Then it fails

    Scenario: wants no file, file does exist
      Given a file "package.json"
      When running:
        """
        has no file *.json
        """
      Then it fails

    Scenario: wants no file, file does not exist
      When running:
        """
        has no file *.json
        """
      Then it succeeds

  Rule: searching by subdirectory glob finds files in subdirectories

    Scenario: wants file, file exists
      Given a file "sub/dir/package.json"
      When running:
        """
        has file **/*.json
        """
      Then it succeeds

    Scenario: wants file, file does not exist
      When running:
        """
        has file **/*.json
        """
      Then it fails

    Scenario: wants no file, file exists
      Given a file "sub/dir/package.json"
      When running:
        """
        has no file **/*.json
        """
      Then it fails

    Scenario: wants no file, file does not exist
      When running:
        """
        has no file **/*.json
        """
      Then it succeeds

  Rule: searching by subdirectory glob finds files in the root directory

    Scenario: wants file, file exists
      Given a file "package.json"
      When running:
        """
        has file **/*.json
        """
      Then it succeeds

    Scenario: wants no file, file exists
      Given a file "package.json"
      When running:
        """
        has no file **/*.json
        """
      Then it fails
