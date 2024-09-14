use clap::Parser;

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
    #[command(about = "Add a birthdays to your list of birthdays")]
    Add {
        #[arg(
            value_name = "NAME",
            help = "The name of the person that you want to add",
            required = true
        )]
        name: String,
        #[arg(
            value_name = "BIRTHDAY",
            help = "The birthday (YYYY-MM-DD, if there is a year, else MM-DD)",
            required = true
        )]
        birthday: String,
    },
    #[command(about = "Import birthdays from a path")]
    Import {
        #[arg(
            value_name = "PATH",
            help = "The path of the file to import data from.\nEach line of the file should be in the format: `NAME MM-DD`",
            required = true
        )]
        path: String,
        #[arg(
            long,
            help = "Should the program exists in case of a duplicate (default: false)",
            short = 'x'
        )]
        exit_on_dupliate: Option<bool>,
        #[arg(
            long,
            help = "Should the program exists in case of a duplicate (default: true)",
            short = 'c'
        )]
        check_duplicate: Option<bool>,
    },
    #[command(about = "Get today's birthdays")]
    Get {
        #[arg(long, help = "Use a separator between names", short = 's')]
        separator: Option<String>,
    },
    #[command(about = "Rename someone")]
    Rename {
        #[arg(
            value_name = "NAME_FROM",
            help = "The original name of the user to be renamed",
            required = true
        )]
        from: String,
        #[arg(
            value_name = "NAME_TO",
            help = "The new name of the user",
            required = true
        )]
        to: String,
    },
    #[command(about = "List all the saved birthdays")]
    List {
        #[arg(
            long,
            help = "Limit how many days you see the birthday from today",
            short = 'l'
        )]
        limit_days: Option<usize>,
        #[arg(long, help = "Limit how many names you see from today", short = 'L')]
        limit_names: Option<usize>,
    },
    #[command(arg_required_else_help = true, alias = "rm")]
    #[command(about = "Remove someone birthday from your list")]
    Remove {
        #[arg(
            value_name = "NAME",
            help = "The name of the person that you want to remove",
            required = true
        )]
        name: String,
    },
    #[command(arg_required_else_help = true)]
    #[command(
        about = "Read the birthday of somebody. This birthday won't appear when using `get` this year"
    )]
    Read {
        #[arg(
            value_name = "NAME",
            help = "The name of the person that you want to read it's birthday",
            required = true
        )]
        name: String,
    },
}
