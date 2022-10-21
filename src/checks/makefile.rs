use crate::errors::UserError;
use crate::fs::file_content;
use regex::Regex;

pub fn target(name: &str) -> Result<bool, UserError> {
    Ok(Makefile::load()?.has_target(name))
}

#[derive(Debug, PartialEq)]
struct Makefile {
    targets: Vec<Target>,
}

impl Makefile {
    fn has_target(&self, name: &str) -> bool {
        self.targets.iter().any(|target| target.name == name)
    }

    fn load() -> Result<Makefile, UserError> {
        Makefile::parse(&file_content("Makefile".as_ref())?)
    }

    fn parse(text: &str) -> Result<Makefile, UserError> {
        let pattern = &"^[a-z]+:";
        let regex = Regex::new(pattern).map_err(|err| UserError::InvalidRegex {
            pattern: pattern.to_string(),
            guidance: err.to_string(),
        })?;
        let mut result = Makefile { targets: vec![] };
        for line in text.lines() {
            for hit in regex.find_iter(line) {
                result.targets.push(Target {
                    name: hit.as_str()[0..hit.end() - 1].into(),
                });
            }
        }
        Ok(result)
    }
}

#[derive(Debug, PartialEq)]
struct Target {
    name: String,
}

#[cfg(test)]
mod tests {
    mod parse {
        use crate::checks::makefile::{Makefile, Target};

        #[test]
        fn with_targets() {
            let give = r#"
foo: a  # the foo target
\tcontent
bar: b
            "#
            .trim();
            let want = Makefile {
                targets: vec![Target { name: "foo".into() }, Target { name: "bar".into() }],
            };
            let have = Makefile::parse(give).unwrap();
            assert_eq!(have, want);
        }
    }
}