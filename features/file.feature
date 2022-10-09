Feature: detect files

  Rule: filename is required

    Scenario: no name provided
      When running:
        """
        has file
        """
      Then it prints:
        """
        ERROR: no name provided

        Usage: has [no] <target> <name>

        Query files and folders:
        > has [no] file <file name>
        > has [no] folder <folder name>

        Query Git repositories:
        > has [no] branch <branch name>
        > has [no] uncommitted-changes
        > has [no] unpushed-changes

        Query command output
        > has [no] empty-output <command> [args...]

        The "no" argument checks for absence of the given object.
        """

    Scenario: duplicate name
      When running:
        """
        has file foo bar
        """
      Then it prints:
        """
        ERROR: too many arguments

        Usage: has [no] <target> <name>

        Query files and folders:
        > has [no] file <file name>
        > has [no] folder <folder name>

        Query Git repositories:
        > has [no] branch <branch name>
        > has [no] uncommitted-changes
        > has [no] unpushed-changes

        Query command output
        > has [no] empty-output <command> [args...]

        The "no" argument checks for absence of the given object.
        """

  Rule: searching by filename finds the given files

    Scenario: wants file, file exists
      Given a file "package.json"
      When running:
        """
        has file package.json
        """
      Then it succeeds

    Scenario: wants file, file does not exist
      When running:
        """
        has file package.json
        """
      Then it fails

    Scenario: wants no file, file does exist
      Given a file "package.json"
      When running:
        """
        has no file package.json
        """
      Then it fails

    Scenario: wants no file, file does not exist
      When running:
        """
        has no file package.json
        """
      Then it succeeds

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

    Scenario: wants no file, file does exist
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

    Scenario: wants file, file does not exist
      When running:
        """
        has file **/*.json
        """
      Then it fails

    Scenario: wants no file, file does exist
      Given a file "package.json"
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
