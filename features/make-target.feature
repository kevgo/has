Feature: detect Make targets

  Scenario: has Make target
    Given a file "Makefile" with content:
      """
      foo:
      """
    When running:
      """
      has make-target foo
      """
    Then it prints nothing
    And it succeeds

  Scenario: doesn't have Make target
    Given a file "Makefile" with content:
      """
      foo:
      """
    When running:
      """
      has make-target bar
      """
    Then it prints nothing
    And it fails

  Scenario: has no Makefile
    When running:
      """
      has make-target foo
      """
    Then it prints nothing
    And it fails
