use std::path::Path;

use clap::Parser;
use lillatelly::{
    source_target_configuration::SourceTargetConfiguration,
    task_tv_show::{TaskAction, TaskTvShow},
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
        for config in all_configurations {
            match TaskTvShow::new(config) {
                Ok(tts) => match tts.dry_run() {
                    Ok(dr) => {
                        for a in dr {
                            log::info!("{:#?}", a);
                            if let TaskAction::Copy(source, target) = a {
                                let output = std::process::Command::new("/bin/cp")
                                    .arg(format!("{}", source.to_str().unwrap()))
                                    .arg(format!("{}", target.full_path.to_str().unwrap()))
                                    .output();
                                match output {
                                    Ok(o) => log::info!("{:#?}", o),
                                    Err(err) => log::error!(
                                        "Error copying {:#?} to {:#?}: `{}`",
                                        source,
                                        target.full_path,
                                        err
                                    ),
                                }
                            } else {
                                log::warn!("Action {:#?} not supported", a);
                            }
                        }
                    }
                    Err(e) => log::error!("Error, collecting dry run `{e}`"),
                },
                Err(e) => log::error!("Error while constructing task Tv Show: `{e}`"),
            }
        }
    }
}
