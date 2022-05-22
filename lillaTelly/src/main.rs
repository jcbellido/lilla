use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct LillaTellyConfiguration {
    version: u8,
    executed_number_of_times: u64,
}

fn main() {
    println!("Hello, world!");
    let mut conf: LillaTellyConfiguration = match confy::load("LillaTelly") {
        Ok(o) => o,
        Err(e) => panic!("{e}"),
    };
    conf.executed_number_of_times += 1;
    println!(
        "This is the time {} that you executed",
        conf.executed_number_of_times
    );
    match confy::store("LillaTelly", &conf) {
        Ok(_) => {}
        Err(e) => panic!("{e}"),
    }
}
