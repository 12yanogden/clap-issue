use clap::{Arg, ArgGroup, Command};

fn init_command() -> Command {
    Command::new("myprog")
        .arg(
            Arg::new("only_required_for_required_group")
                .long("only_required_for_required_group")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("required_for_dependent_and_required_group")
                .long("required_for_dependent_and_required_group")
                .action(clap::ArgAction::SetTrue),
        )
        .group(
            ArgGroup::new("required_group")
                .args([
                    "only_required_for_required_group",
                    "required_for_dependent_and_required_group",
                ])
                .required(true),
        )
        .arg(
            Arg::new("dependent")
                .long("dependent")
                .action(clap::ArgAction::SetTrue),
        )
        .group(
            ArgGroup::new("requires_group")
                .args(["dependent"])
                .requires("required_for_dependent_and_required_group"),
        )
}

fn main() {
    let cmd = init_command();
    let matches = cmd.get_matches();
    println!("{:?}", matches);
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::error::ErrorKind;

    lazy_static::lazy_static! {
        static ref CMD: Command = init_command();
    }

    #[test]
    fn test_missing_required_argument_error() {
        let result = CMD.clone().try_get_matches_from(vec![
            "myprog",
            "--only_required_for_required_group",
            "--dependent",
        ]);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::MissingRequiredArgument);
    }

    #[test]
    fn test_has_required_argument() {
        let result = CMD.clone().try_get_matches_from(vec![
            "myprog",
            "--required_for_dependent_and_required_group",
            "--dependent",
        ]);

        assert!(result.is_ok());
    }
}
