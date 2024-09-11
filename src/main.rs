use {
    clap::Parser,
    hbd::cmd::{add, get},
};

fn main() {
    let args = hbd::cli::Cli::parse();

    match args.command {
        hbd::cli::Commands::Get => get(),
        hbd::cli::Commands::Add { name, birthday } => add(&name, &birthday),
        _ => Ok(()),
    }
    .unwrap();
}
