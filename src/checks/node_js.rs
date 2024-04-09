use crate::errors::UserError;
use crate::fs::file_content;
use serde::Deserialize;
use std::collections::HashMap;

pub fn has_dependency(name: &str) -> Result<bool, UserError> {
  match file_content("package.json".as_ref()) {
    Ok(text) => Ok(PackageJson::parse(&text)?.has_dependency(name)),
    Err(_) => Ok(false),
  }
}

pub fn has_dev_dependency(name: &str) -> Result<bool, UserError> {
  match file_content("package.json".as_ref()) {
    Ok(text) => Ok(PackageJson::parse(&text)?.has_dev_dependency(name)),
    Err(_) => Ok(false),
  }
}

#[derive(Debug, Deserialize, PartialEq)]
struct PackageJson {
  dependencies: Option<HashMap<String, String>>,
  #[serde(rename = "devDependencies")]
  dev_dependencies: Option<HashMap<String, String>>,
}

impl PackageJson {
  fn has_dependency(&self, name: &str) -> bool {
    match &self.dependencies {
      Some(dependencies) => dependencies.contains_key(name),
      None => false,
    }
  }

  fn has_dev_dependency(&self, name: &str) -> bool {
    match &self.dev_dependencies {
      Some(dev_dependencies) => dev_dependencies.contains_key(name),
      None => false,
    }
  }

  fn parse(text: &str) -> Result<PackageJson, UserError> {
    serde_json::from_str(text).map_err(|err| UserError::InvalidPackageJsonStructure {
      guidance: err.to_string(),
    })
  }
}

#[cfg(test)]
mod tests {
  mod package_json {
    use crate::checks::node_js::PackageJson;
    use big_s::S;
    use std::collections::HashMap;

    #[test]
    fn has_dependency() {
      let package_json = PackageJson {
        dependencies: Some(HashMap::from([(S("alpha"), S("1.0.0"))])),
        dev_dependencies: None,
      };
      assert!(package_json.has_dependency("alpha"));
      assert!(!package_json.has_dependency("zonk"));
    }

    #[test]
    fn has_dev_dependency() {
      let package_json = PackageJson {
        dependencies: None,
        dev_dependencies: Some(HashMap::from([(S("alpha"), S("1.0.0"))])),
      };
      assert!(package_json.has_dev_dependency("alpha"));
      assert!(!package_json.has_dev_dependency("zonk"));
    }

    mod parse {
      use crate::checks::node_js::PackageJson;
      use big_s::S;
      use std::collections::HashMap;

      #[test]
      fn valid_package_json_file() {
        let give = r#"
                    {
                        "name": "foo",
                        "dependencies": {
                            "alpha": "1.0.0",
                            "beta": "2.0.0"
                        },
                        "devDependencies": {
                            "gamma": "3.0.0",
                            "delta": "4.0.0"
                        }
                    }"#;
        let want = Ok(PackageJson {
          dependencies: Some(HashMap::from([
            (S("alpha"), S("1.0.0")),
            (S("beta"), S("2.0.0")),
          ])),
          dev_dependencies: Some(HashMap::from([
            (S("gamma"), S("3.0.0")),
            (S("delta"), S("4.0.0")),
          ])),
        });
        let have = PackageJson::parse(give);
        assert_eq!(have, want);
      }

      #[test]
      fn invalid_json() {
        let give = r#"
                    {
                        "name": "foo
                    "#;
        let have = PackageJson::parse(give);
        assert!(have.is_err());
      }

      #[test]
      fn empty_file() {
        let give = r"";
        let have = PackageJson::parse(give);
        assert!(have.is_err());
      }
    }
  }
}
