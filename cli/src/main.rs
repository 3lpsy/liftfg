use anyhow::Result;
use clap::{Arg, ArgMatches, Command};
use db::db::migrate;
use db::db::rollback;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

// use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};
#[tokio::main]
async fn main() -> Result<()> {
    let args = parse();

    match args.subcommand() {
        Some(("ui", gargs)) => match gargs.subcommand() {
            Some(("migrate", _sargs)) => {
                println!("Migrate!");
                let db = args.get_one::<String>("db");
                migrate(db).await?
            }
            Some(("rollback", _sargs)) => {
                println!("Rollback!");
                let db = args.get_one::<String>("db");
                rollback(db).await?
            }
            _ => unreachable!("No more subcommands"),
        },
        _ => unreachable!("No more subcommands"),
    }
    Ok(())
}

pub fn parse() -> ArgMatches {
    app_command().get_matches()
}

fn app_command() -> Command {
    Command::new(PKG_NAME)
        .version(VERSION)
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(ui_command())
}

fn ui_command() -> Command {
    Command::new("ui")
        .about("UI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(migrate_command())
        .subcommand(rollback_command())
        .arg(
            Arg::new("db")
                .short('d')
                .long("db")
                .global(true)
                .required(false),
        )
}

fn migrate_command() -> Command {
    Command::new("migrate").about("Migrate database")
}
fn rollback_command() -> Command {
    Command::new("rollback").about("Rollback database")
}
