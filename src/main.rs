use {
    clap::Parser,
    hbd::cmd::{add, get, list, remove},
};

fn main() {
    let args = hbd::cli::Cli::parse();

    match args.command {
        hbd::cli::Commands::Get => get(),
        hbd::cli::Commands::Add { name, birthday } => add(&name, &birthday),
        hbd::cli::Commands::List {
            limit_days,
            limit_names,
        } => list(limit_days, limit_names),
        hbd::cli::Commands::Remove { name } => remove(&name),
        _ => Ok(()),
    }
    .unwrap();
}
