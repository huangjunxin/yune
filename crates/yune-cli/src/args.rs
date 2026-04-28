use std::path::PathBuf;

use crate::default_sequence;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Command {
    Run { sequence: String },
    Check { fixture: PathBuf },
    Help,
}

impl Command {
    pub(crate) fn parse(args: &[String]) -> Result<Self, String> {
        match args.first().map(String::as_str) {
            None => Ok(Self::Run {
                sequence: default_sequence().to_owned(),
            }),
            Some("run") => Ok(Self::Run {
                sequence: args
                    .get(1)
                    .map_or_else(|| default_sequence().to_owned(), ToOwned::to_owned),
            }),
            Some("check") => {
                let fixture = args
                    .get(1)
                    .ok_or_else(|| "usage: yune-cli check <fixture.json>".to_owned())?;
                Ok(Self::Check {
                    fixture: PathBuf::from(fixture),
                })
            }
            Some("-h" | "--help" | "help") => Ok(Self::Help),
            Some(command) => Err(format!("unknown command: {command}\n\n{}", help_text())),
        }
    }
}

pub(crate) fn help_text() -> &'static str {
    "usage:\n  yune-cli run [key-sequence]\n  yune-cli check <fixture.json>"
}

#[cfg(test)]
mod tests {
    use super::Command;

    #[test]
    fn default_command_runs_default_sequence() {
        assert_eq!(
            Command::parse(&[]),
            Ok(Command::Run {
                sequence: "nihao ".to_owned()
            })
        );
    }

    #[test]
    fn parses_check_command() {
        assert_eq!(
            Command::parse(&["check".to_owned(), "fixture.json".to_owned()]),
            Ok(Command::Check {
                fixture: "fixture.json".into()
            })
        );
    }
}
