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
    Get,
    List {
        #[arg(
            long,
            help = "Limit how many days you see the birthday from today",
            short = 'l'
        )]
        limit_days: Option<u16>,
        #[arg(long, help = "Limit how many names you see from today", short = 'L')]
        limit_names: Option<u16>,
    },
    #[command(arg_required_else_help = true, alias = "rm")]
    Remove {
        #[arg(
            value_name = "NAME",
            help = "The name of the person that you want to remove",
            required = true
        )]
        name: String,
    },
    #[command(arg_required_else_help = true)]
    Read {
        #[arg(
            value_name = "NAME",
            help = "The name of the person that you want to read it's birthday",
            required = true
        )]
        name: String,
    },
}
