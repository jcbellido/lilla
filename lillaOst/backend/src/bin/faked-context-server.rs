use backend::servers::faked_context_server;

#[tokio::main]
async fn main() {
    println!(">>> Trying to load `.dev_env` dotenv environment");
    match dotenv::from_filename(".dev_env") {
        Ok(_) => {}
        Err(e) => panic!(".dev_env file not found, aborting: `{:#?}`", e),
    };
    pretty_env_logger::init();

    log::info!("Starting server with FAKE data");
    faked_context_server::faked_context_server().await;
}
