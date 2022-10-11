Feature: detect empty command output

  Scenario: wants output, gets output
    When running:
      """
      has command-output echo Hello world!
      """
    Then it succeeds

  Scenario: wants output, gets no output
    When running:
      """
      has command-output echo
      """
    Then it fails

  Scenario: wants output, gets output containing only newlines
    When running:
      """
      has command-output printf \n\n
      """
    Then it fails

  Scenario: wants no output, gets no output
    When running:
      """
      has no command-output echo
      """
    Then it succeeds

  Scenario: wants no output, gets output containing only newlines
    When running:
      """
      has no command-output printf \n\n
      """
    Then it succeeds

  Scenario: wants no output, gets output
    When running:
      """
      has no command-output echo Hello world!
      """
    Then it fails
