Feature: detect active Git branches

  Scenario Outline:
    Given a Git repo
    And my Git workspace has a branch "<BRANCH>"
    And my Git workspace is on the "<ACTIVE>" branch
    When running "<QUERY>"
    Then it signals <RESULT>
    And it prints nothing

    Examples:
      | DESCRIPTION                        | QUERY                            | BRANCH  | ACTIVE  | RESULT   |
      | matching branch is checked out     | has git-branch-active feature    | feature | feature | match    |
      | negation                           | has no git-branch-active feature | feature | feature | no match |
      | matching branch is not checked out | has git-branch-active feature    | feature | main    | no match |
      | matching branch does not exist     | has git-branch-active other      | feature | feature | no match |
