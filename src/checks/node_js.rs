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

#[derive(Debug, Deserialize, PartialEq)]
struct PackageJson {
    dependencies: HashMap<String, String>,
}

impl PackageJson {
    fn has_dependency(&self, name: &str) -> bool {
        self.dependencies.contains_key(name)
    }

    fn parse(text: &str) -> Result<PackageJson, UserError> {
        serde_json::from_str(text).map_err(|err| UserError::InvalidePackageJsonStructure {
            guidance: err.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    mod package_json {
        use std::collections::HashMap;

        use crate::checks::node_js::PackageJson;

        mod parse {
            use crate::checks::node_js::PackageJson;
            use std::collections::HashMap;

            #[test]
            fn valid() {
                let give = r#"
{
    "name": "foo",
    "dependencies": {
        "alpha": "1.0.0",
        "beta": "2.0.0"
    }
}"#;
                let want = Ok(PackageJson {
                    dependencies: HashMap::from([
                        ("alpha".into(), "1.0.0".into()),
                        ("beta".into(), "2.0.0".into()),
                    ]),
                });
                let have = PackageJson::parse(give);
                assert_eq!(have, want);
            }

            #[test]
            fn invalid() {
                let give = r#"
{
    "name": "foo
"#;
                let have = PackageJson::parse(give);
                assert!(have.is_err());
            }

            #[test]
            fn empty() {
                let give = r#""#;
                let have = PackageJson::parse(give);
                assert!(have.is_err());
            }
        }

        #[test]
        fn has_dependency() {
            let package_json = PackageJson {
                dependencies: HashMap::from([("alpha".into(), "1.0.0".into())]),
            };
            assert!(package_json.has_dependency("alpha"));
            assert!(!package_json.has_dependency("zonk"));
        }
    }
}
