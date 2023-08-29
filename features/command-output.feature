Feature: detect existence of command output

  Rule: if output is expected, the given command must print something

    Scenario: the given command prints something
      When running:
        """
        has command-output echo Hello world!
        """
      Then it succeeds
      And it prints nothing

    Scenario: the given command prints no output
      When running:
        """
        has command-output echo
        """
      Then it fails
      And it prints nothing

    Scenario: the given command prints only newlines
      When running:
        """
        has command-output printf "\n\n"
        """
      Then it fails
      And it prints nothing

  Rule: if no output is expected, the given command must not print anything

    Scenario: the given command prints nothing
      When running:
        """
        has no command-output echo
        """
      Then it succeeds
      And it prints nothing

    Scenario: the given command prints only newlines
      When running:
        """
        has no command-output printf "\n\n"
        """
      Then it succeeds
      And it prints nothing

    Scenario: the given command prints something
      When running:
        """
        has no command-output echo Hello world!
        """
      Then it fails
      And it prints nothing
