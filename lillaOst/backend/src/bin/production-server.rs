use backend::servers::production_server::production_server;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    {
        println!(">>> Trying to load `.dev_env` dotenv environment");
        match dotenv::from_filename(".dev_env") {
            Ok(_) => {}
            Err(e) => panic!(".dev_env file not found, aborting: `{:#?}`", e),
        };
    }
    #[cfg(not(debug_assertions))]
    {
        println!(">>> Trying to load production `.env` dotenv environment");
        dotenv::dotenv().ok();
    }

    pretty_env_logger::init();
    log::info!("Starting production server");
    production_server().await;
}
