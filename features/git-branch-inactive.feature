Feature: detect inactive Git branches

  Scenario Outline:
    Given a Git repo
    And my Git workspace has a branch "<BRANCH>"
    And my Git workspace is on the "<ACTIVE>" branch
    When running "<QUERY>"
    Then it signals <RESULT>
    And it prints nothing

    Examples:
      | DESCRIPTION                        | QUERY                              | BRANCH  | ACTIVE  | RESULT   |
      | matching branch is not checked out | has git-branch-inactive feature    | feature | main    | match    |
      | negation                           | has no git-branch-inactive feature | feature | main    | no match |
      | matching branch is checked out     | has git-branch-inactive feature    | feature | feature | no match |
      | no matching branch                 | has git-branch-inactive other      | feature | feature | no match |
