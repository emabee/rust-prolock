//! Hides all the clap stuff, exposes the commandline as struct Args.

use clap::{Arg, ArgAction, ArgMatches, Command, crate_description};

#[allow(clippy::let_and_return)]
#[allow(clippy::too_many_lines)]
fn get_clap_command() -> Command {
    let command = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(crate_description!())
        .help_template(
            "\
{before-help}{name} {version}

{about}
Authors: {author-with-newline}
{usage-heading} {usage}

{all-args}
{after-help}",
        )
        .arg(
            Arg::new("test")
                .long("test")
                .help("test-mode for Prolock; uses different file locations.")
                .required(false)
                .num_args(0)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("use_file")
                .short('f')
                .long("use_file")
                .value_name("file")
                .help("Prolock file to open; if not given, the last file is opened, or, on first start, a default file is chosen.")
                .required(false)
                .num_args(1)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("forget_file")
                .long("forget_file")
                .value_name("file")
                .help("Remove the file from Prolock's list of known files.")
                .required(false)
                .number_of_values(1)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("list_known_files")
                .long("list_known_files")
                .short('l')
                .help("Print Prolock's list of known files.")
                .number_of_values(0)
                .required(false),
        );

    command
}

#[derive(Default, Eq, PartialEq, Debug)]
pub(crate) struct Args {
    arg_matches: ArgMatches,
}

impl Args {
    pub fn from_command_line() -> Args {
        Self::from(get_clap_command().get_matches())
    }

    pub fn from(arg_matches: ArgMatches) -> Self {
        Args { arg_matches }
    }

    pub fn is_test(&self) -> bool {
        self.arg_matches.get_flag("test")
    }

    pub fn file(&self) -> Option<&String> {
        self.arg_matches.get_one::<String>("use_file")
    }
    pub fn list_known_files(&self) -> bool {
        self.arg_matches.get_flag("list_known_files")
    }
    pub fn forget_file(&self) -> Option<&String> {
        self.arg_matches.get_one::<String>("forget_file")
    }
}

#[cfg(test)]
mod test {
    use super::{Args, get_clap_command};

    fn args_from<I, T>(itr: I) -> Args
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        Args::from(get_clap_command().try_get_matches_from(itr).unwrap())
    }

    #[test]
    fn test_command_lines() {
        {
            let args = args_from(Vec::<String>::new());
            assert!(args.file().is_none());
        }
        {
            let args = args_from(vec!["prolock", "--forget_file", "my_file0"]);
            assert_eq!(args.forget_file().unwrap().as_str(), "my_file0");
        }
        {
            let args = args_from(vec!["prolock", "-f", "my_file1"]);
            assert_eq!(args.file().unwrap().as_str(), "my_file1");
        }
        {
            let args = args_from(vec!["prolock", "--use_file", "my_file2"]);
            assert_eq!(args.file().unwrap().as_str(), "my_file2");
        }
        {
            let args = args_from(vec!["prolock", "--list_known_files"]);
            assert!(args.list_known_files());
        }
    }

    #[test]
    #[should_panic = "called `Result::unwrap()` on an `Err` value"]
    fn test_bad_command_line1() {
        args_from(vec!["prolock", "--use_file"]);
    }
    #[test]
    #[should_panic = "called `Result::unwrap()` on an `Err` value"]
    fn test_bad_command_line2() {
        args_from(vec!["prolock", "--forget_file"]);
    }
    #[test]
    #[should_panic = "called `Result::unwrap()` on an `Err` value"]
    fn test_bad_command_line3() {
        args_from(vec!["prolock", "-f", "my_file0", "--use_file", "my_file1"]);
    }
}
