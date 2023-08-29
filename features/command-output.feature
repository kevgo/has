Feature: detect command output

  Scenario Outline:
    When running:
      """
      <QUERY>
      """
    Then it signals <RESULT>
    And it prints nothing

    Examples:
      | DESCRIPTION                  | QUERY                                | RESULT   |
      | commands prints something    | has command-output echo Hello world! | match    |
      | command prints nothing       | has command-output echo              | no match |
      | negation                     | has no command-output echo           | match    |
      | command prints only newlines | has command-output printf "\n\n"     | no match |

  Scenario: wrong arguments
    When running "has command-output"
    Then it fails
    And the output starts with "ERROR: missing command to run"
