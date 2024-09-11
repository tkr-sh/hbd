use clap::{
    error::ErrorKind,
    value_parser,
    Arg,
    ArgAction,
    ArgGroup,
    ArgMatches,
    Command,
    Parser,
    ValueEnum,
};

#[derive(Debug, Parser)]
#[command(name = "hbd")]
#[command(about = "A command line tool to help you remind birthdays", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    Add {
        #[arg(
            value_name = "NAME",
            help = "The name of the person that you want to add"
        )]
        #[arg(required = true)]
        name: String,
        #[arg(
            value_name = "BIRTHDAY",
            help = "The birthday (YYYY-MM-DD, if there is a year, else MM-DD)"
        )]
        #[arg(required = true)]
        birthday: String,
    },
    Get,
    List {},
    Remove {},
    //     /// Clones repos
    //     #[command(arg_required_else_help = true)]
    //     Clone {
    //         /// The remote to clone
    //         remote: String,
    //     },
    //     /// Compare two commits
    //     Diff {
    //         #[arg(value_name = "COMMIT")]
    //         base: Option<OsString>,
    //         #[arg(value_name = "COMMIT")]
    //         head: Option<OsString>,
    //         #[arg(last = true)]
    //         path: Option<OsString>,
    //         #[arg(
    //             long,
    //             require_equals = true,
    //             value_name = "WHEN",
    //             num_args = 0..=1,
    //             default_value_t = ColorWhen::Auto,
    //             default_missing_value = "always",
    //             value_enum
    //         )]
    //         color: ColorWhen,
    //     },
    //     /// pushes things
    //     #[command(arg_required_else_help = true)]
    //     Push {
    //         /// The remote to target
    //         remote: String,
    //     },
    //     /// adds things
    //     #[command(arg_required_else_help = true)]
    //     Add {
    //         /// Stuff to add
    //         #[arg(required = true)]
    //         path: Vec<PathBuf>,
    //     },
    //     Stash(StashArgs),
    //     #[command(external_subcommand)]
    //     External(Vec<OsString>),
}
