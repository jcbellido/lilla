use std::path::Path;

use clap::Parser;

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
    let args = Args::parse();

    let config_file = Path::new(&args.config);

    if !config_file.exists() || !config_file.is_file() {
        panic!("Config file `{}` can't be found!", args.config);
    }

    if args.dry && args.force {
        panic!("Can't have both `dry` and `force`");
    }


    let all_configurations =
        serde_json::from_str::<Vec<SourceTargetConfiguration>>(source).unwrap();

    println!("args: `{:#?}`", args);
}
