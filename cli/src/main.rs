use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;
use clap::{Arg, ArgMatches, Command};
use db::db::migrate;
use db::db::rollback;
use fgcore::logging;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

// use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};
#[tokio::main]
async fn main() -> Result<()> {
    logging::setup()?;
    let args = parse();

    match args.subcommand() {
        Some(("migrate", _sargs)) => {
            println!("Migrate!");
            let db = args.get_one::<String>("db");
            if db.is_some() {
                migrate(&PathBuf::from_str(db.unwrap()).unwrap()).await?
            }
        }
        Some(("rollback", _sargs)) => {
            println!("Rollback!");
            let db = args.get_one::<String>("db");
            if db.is_some() {
                rollback(&PathBuf::from_str(db.unwrap()).unwrap()).await?
            }
        }
        // Some(("entities", _sargs)) => {
        //     println!("Generating entities!");
        //     let db = args.get_one::<String>("db");
        //     let output = args.get_one::<String>("output");

        //     if db.is_some() {
        //         generate(
        //             &PathBuf::from_str(db.unwrap()).unwrap(),
        //             &PathBuf::from_str(output.unwrap()).unwrap(),
        //         )
        //         .await?
        //     }
        // }
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
        .arg(
            Arg::new("db")
                .short('d')
                .long("db")
                .global(true)
                .required(false),
        )
        .subcommand(migrate_command())
        .subcommand(rollback_command())
    // .subcommand(entities_command())
}

// fn entities_command() -> Command {
//     Command::new("entities")
//         .about("Generate Entities")
//         .arg(Arg::new("output").short('o').long("output").required(true))
// }

fn migrate_command() -> Command {
    Command::new("migrate").about("Migrate database")
}
fn rollback_command() -> Command {
    Command::new("rollback").about("Rollback database")
}
