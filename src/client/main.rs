mod commands;
mod config;
mod context;
mod error;
mod fs;
mod run;
mod tree;
mod ui;
mod util;

use std::path::PathBuf;
use std::process;

use lazy_static::lazy_static;
use structopt::StructOpt;

use crate::config::{AppConfig, AppKeyMapping, AppTheme, ConfigStructure};
use crate::context::AppContext;
use crate::error::AppResult;
use crate::run::run;
use crate::tree::DbusTreeTrait;

const PROGRAM_NAME: &str = "tsuchita";
const CONFIG_FILE: &str = "tsuchita.toml";
const KEYMAP_FILE: &str = "keymap.toml";
const THEME_FILE: &str = "theme.toml";

lazy_static! {
    // dynamically builds the config hierarchy
    static ref CONFIG_HIERARCHY: Vec<PathBuf> = {
        let mut temp = vec![];
        match xdg::BaseDirectories::with_prefix(PROGRAM_NAME) {
            Ok(dirs) => temp.push(dirs.get_config_home()),
            Err(e) => eprintln!("{}", e),
        };
        // adds the default config files to the config hierarchy if running through cargo
        if cfg!(debug_assertions) {
            temp.push(PathBuf::from("./config"));
        }
        temp
    };

    static ref THEME_T: AppTheme = AppTheme::get_config(THEME_FILE);
}

#[derive(Clone, Debug, StructOpt)]
pub struct Args {
    #[structopt(short = "v", long = "version")]
    version: bool,
}

fn run_tsuchita(args: Args) -> AppResult<()> {
    if args.version {
        let version = env!("CARGO_PKG_VERSION");
        println!("{}", version);
        return Ok(());
    }

    let config = AppConfig::get_config(CONFIG_FILE);
    let keymap = AppKeyMapping::get_config(KEYMAP_FILE);

    eprintln!("config: {:#?}", config);

    let mut context = AppContext::new(config);
    let url = context.config_ref().server_ref().url.clone();

    let display_options = context
        .config_ref()
        .client_ref()
        .display_options_ref()
        .clone();
    context
        .tree_mut()
        .fetch_sources(url.as_str(), &display_options)?;

    let curr_source = context
        .tree_ref()
        .get("/")
        .and_then(|list| list.curr_entry_ref())
        .and_then(|entry| Some(entry.name().to_string()));

    if let Some(source) = curr_source.as_ref() {
        context
            .tree_mut()
            .fetch_messages(url.as_str(), source, &display_options)?;
    }

    let mut backend: ui::TuiBackend = ui::TuiBackend::new()?;
    run(&mut backend, &mut context, keymap)?;

    Ok(())
}

fn main() {
    let args = Args::from_args();

    match run_tsuchita(args) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e.to_string());
            process::exit(1);
        }
    }
}
