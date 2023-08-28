Feature: detect Git commit authors

  Background:

  Scenario: wants commit by author, has commit by author
    Given a Git repo with the user "John Doe" and email "jd@acme.com"
    When running:
      """
      has author jd@acme.com
      """
    Then it succeeds
    And it prints nothing

  Scenario: wants commit by author, doesn't have commit by author
    Given a Git repo with the user "Somebody Else" and email "other@acme.com"
    When running:
      """
      has author kevgo
      """
    Then it fails
    And it prints nothing

  Scenario: wants no commits by author, has commit by author
    Given a Git repo with the user "John Doe" and email "jd@acme.com"
    When running:
      """
      has no author kevgo
      """
    Then it fails
    And it prints nothing

  Scenario: wants no commits by author, has no commits by author
    Given my Git workspace has a commit by "other"
    And my Git workspace is on the "main" branch
    When running:
      """
      has no author kevgo
      """
    Then it succeeds
    And it prints nothing
