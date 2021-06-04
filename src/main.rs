mod list;
mod install;
mod update;
mod profile;

use std::{ fs, io };
use argh::FromArgs;
use profile::Config;


/// The Sek Shell
#[derive(FromArgs)]
struct Options {
    #[argh(subcommand)]
    command: Command
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum Command {
    List(list::Options),
    Install(install::Options),
    Update(update::Options)
}

fn main() -> anyhow::Result<()> {
    let options: Options = argh::from_env();

    let config = Config::new()?;

    fs::create_dir_all(config.projdir.data_dir())
        .or_else(|err| if err.kind() == io::ErrorKind::AlreadyExists {
            Ok(())
        } else {
            Err(err)
        })?;

    match options.command {
        Command::List(cmd) => cmd.exec(&config),
        Command::Install(cmd) => cmd.exec(&config),
        Command::Update(cmd) => cmd.exec(&config)
    }
}