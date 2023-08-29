Feature: detect Git commit authors

  Scenario Outline:
    Given a Git repo with the user "<NAME>" and email "<EMAIL>"
    And a local commit
    When running:
      """
      <QUERY>
      """
    Then it signals <RESULT>
    And it prints nothing

    Examples:
      | DESCRIPTION          | NAME          | EMAIL       | QUERY                                    | RESULT   |
      | match by email       | John Doe      | jd@acme.com | has git-commits-by-author jd@acme.com    | match    |
      | match by name        | John Doe      | jd@acme.com | has git-commits-by-author "John Doe"     | match    |
      | no commits by author | Somebody Else | se@acme.com | has git-commits-by-author jd@acme.com    | no match |
      | match by email       | John Doe      | jd@acme.com | has no git-commits-by-author jd@acme.com | no match |
