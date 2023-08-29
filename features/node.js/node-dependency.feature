Feature: detect Node depenndencies

  Scenario Outline:
    Given a file "package.json" with content:
      """
      {
        "name": "foo",
        "dependencies": {
          "alpha": "1.0.0.",
          "beta": "2.0.0"
        },
        "devDependencies": {
          "gamma": "2.0.0"
        }
      }
      """
    When running "<QUERY>"
    Then it signals <RESULT>
    And it prints nothing

    Examples:
      | DESCRIPTION                                  | QUERY                           | RESULT   |
      | matching dependency                          | has nodejs-dependency beta      | match    |
      | matching dependency is a dev-dependency      | has nodejs-dependency gamma     | no match |
      | dependency doesn't exist                     | has nodejs-dependency zonk      | no match |
      | matching dev-dependency                      | has nodejs-dev-dependency gamma | match    |
      | matching dev-dependency is a prod dependency | has nodejs-dev-dependency beta  | no match |
      | dev-dependency doesn't exist                 | has nodejs-dev-dependency zonk  | no match |
