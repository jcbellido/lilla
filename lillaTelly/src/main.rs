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

    #[clap(short, long)]
    force: bool,
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

    if args.dry && args.force {
        log::error!("Can't have both `dry` and `force`");
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
                Ok(tts) => match tts.dry_run() {
                    Ok(dr) => {
                        log::info!(
                            "Actions gatehered for {} - {}",
                            tts.configuration.source,
                            tts.configuration.target
                        );
                        for a in dr {
                            log::info!("{:#?}", a);
                        }
                    }
                    Err(e) => log::error!("Error, collecting dry run `{e}`"),
                },
                Err(e) => log::error!("Error while constructing task Tv Show: `{e}`"),
            }
        }
    } else {
        log::warn!("Non dry runs are still unimplemented, sorry!");
    }
}
