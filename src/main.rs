use {
    clap::Parser,
    hbd::cmd::{add, get, import, list, read, remove, rename, set},
};

fn main() {
    let args = hbd::cli::Cli::parse();

    match args.command {
        hbd::cli::Commands::Add { name, birthday } => add(&name, &birthday),
        hbd::cli::Commands::Get { separator } => get(separator),
        hbd::cli::Commands::Import {
            path,
            exit_on_dupliate,
            check_duplicate,
        } => import(&path, exit_on_dupliate, check_duplicate),
        hbd::cli::Commands::List {
            limit_days,
            limit_names,
            descending,
            separator_days,
        } => {
            list(
                limit_days,
                limit_names,
                descending.unwrap_or(false),
                separator_days,
            )
        },
        hbd::cli::Commands::Read { name } => read(&name),
        hbd::cli::Commands::Remove { name } => remove(&name),
        hbd::cli::Commands::Rename { from, to } => rename(&from, &to),
        hbd::cli::Commands::Set { name, birthday } => set(&name, &birthday),
    }
    .unwrap();
}
