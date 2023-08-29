Feature: detect Git commit authors

  Scenario: wants commit by author email, has commit by author
    Given a Git repo with the user "John Doe" and email "jd@acme.com"
    And a local commit
    When running:
      """
      has git-commits-by-author jd@acme.com
      """
    Then it succeeds
    And it prints nothing

  Scenario: wants commit by author name, has commit by author
    Given a Git repo with the user "John Doe" and email "jd@acme.com"
    And a local commit
    When running:
      """
      has git-commits-by-author "John Doe"
      """
    Then it succeeds
    And it prints nothing

  Scenario: wants commit by author, doesn't have commit by author
    Given a Git repo with the user "Somebody Else" and email "other@acme.com"
    And a local commit
    When running:
      """
      has git-commits-by-author jd@acme.com
      """
    Then it fails
    And it prints nothing

  Scenario: wants no commits by author, has commit by author
    Given a Git repo with the user "John Doe" and email "jd@acme.com"
    And a local commit
    When running:
      """
      has no git-commits-by-author jd@acme.com
      """
    Then it fails
    And it prints nothing

  Scenario: wants no commits by author, has no commits by author
    Given a Git repo with the user "Somebody Else" and email "other@acme.com"
    And a local commit
    And my Git workspace is on the "main" branch
    When running:
      """
      has no git-commits-by-author jd@acme.com
      """
    Then it succeeds
    And it prints nothing
