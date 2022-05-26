use std::path::Path;

use clap::Parser;
use lillatelly::{
    source_target_configuration::SourceTargetConfiguration, task_tv_show::TaskTvShow,
};

#[derive(Parser, Debug)]
#[clap(version, long_about = None)]
struct Args {
    #[clap(short, long)]
    config: String,

    #[clap(short, long)]
    dry: bool,
}

fn main() {
    println!("Remember to setup RUST_LOG=\"debug\"");
    pretty_env_logger::init();

    let args = Args::parse();
    let config_file = Path::new(&args.config);

    if !config_file.exists() || !config_file.is_file() {
        log::error!("Config file `{}` can't be found!", args.config);
        return;
    }

    log::info!("Read args: `{:#?}`", args);

    let file = std::fs::File::open(config_file).expect("Had problems opening the file!");
    let reader = std::io::BufReader::new(file);

    let all_configurations: Vec<SourceTargetConfiguration> =
        serde_json::from_reader(reader).expect("Had problems parsing the configuration file");

    if args.dry {
        for config in all_configurations {
            match TaskTvShow::new(config) {
                Ok(tts) => {
                    if let Err(err) = tts.dry_run() {
                        log::error!("Error during dry run: `{err}`");
                    }
                }
                Err(e) => log::error!("Error while constructing task Tv Show: `{e}`"),
            }
        }
    } else {
        for config in all_configurations {
            match TaskTvShow::new(config) {
                Ok(tts) => {
                    if let Err(err) = tts.run() {
                        log::error!("Error executing TV Show task: `{err}`");
                    }
                }
                Err(e) => log::error!("Error while constructing task Tv Show: `{e}`"),
            }
        }
    }
}
