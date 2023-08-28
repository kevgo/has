Feature: detect file by glob

  Rule: simple globs match only files in the current directory

    Scenario: matching file in the current directory
      Given a file "package.json"
      When running:
        """
        has file *.json
        """
      Then it succeeds
      And it prints nothing

    Scenario: no matching file in the current directory

  Rule: double asterisk globs match files in directories

    Scenario: matching file in subdirectory
      Given a file "sub/dir/package.json"
      When running:
        """
        has file **/*.json
        """
      Then it succeeds
      And it prints nothing

    Scenario: wants no file, file exists
      Given a file "sub/dir/package.json"
      When running:
        """
        has no file **/*.json
        """
      Then it fails
      And it prints nothing

    Scenario: wants no file, file does not exist
      When running:
        """
        has no file **/*.json
        """
      Then it succeeds
      And it prints nothing

  Rule: searching by subdirectory glob finds files in the root directory

    Scenario: wants file, file exists
      Given a file "package.json"
      When running:
        """
        has file **/*.json
        """
      Then it succeeds
      And it prints nothing

    Scenario: wants no file, file exists
      Given a file "package.json"
      When running:
        """
        has no file **/*.json
        """
      Then it fails
      And it prints nothing
