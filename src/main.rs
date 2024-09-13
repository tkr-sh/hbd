use {
    clap::Parser,
    hbd::cmd::{add, get, list, read, remove},
};

fn main() {
    let args = hbd::cli::Cli::parse();

    match args.command {
        hbd::cli::Commands::Get { separator } => get(separator),
        hbd::cli::Commands::Read { name } => read(&name),
        hbd::cli::Commands::Add { name, birthday } => add(&name, &birthday),
        hbd::cli::Commands::List {
            limit_days,
            limit_names,
        } => list(limit_days, limit_names),
        hbd::cli::Commands::Remove { name } => remove(&name),
    }
    .unwrap();
}
