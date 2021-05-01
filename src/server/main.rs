mod config;
mod dbus_monitor;
mod message;
mod server;

use std::path::PathBuf;
use std::thread;

use lazy_static::lazy_static;
use structopt::StructOpt;

use crate::config::{AppConfig, ConfigStructure};

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
}
#[derive(Clone, Debug, StructOpt)]
pub struct Args {
    #[structopt(long = "path", parse(from_os_str))]
    path: Option<PathBuf>,
    #[structopt(short = "v", long = "version")]
    version: bool,
    #[structopt(long = "lastdir", parse(from_os_str))]
    last_dir: Option<PathBuf>,
}

async fn run_tsuchita(args: Args) -> std::io::Result<()> {
    if args.version {
        let version = env!("CARGO_PKG_VERSION");
        println!("{}", version);
        return Ok(());
    }

    let config = AppConfig::get_config(CONFIG_FILE);

    println!("Listening to dbus...");
    let _dbus_thread = thread::spawn(|| {
        let res = dbus_monitor::listen();
        match res {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    });

    println!("Running HTTP server on {}", config.server_ref().url);
    server::serve(&config).await;

    Ok(())
}

#[actix_web::main]
async fn main() {
    let args = Args::from_args();

    run_tsuchita(args).await;
}
