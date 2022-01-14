use std::env;

use warp::Filter;

use tokio::sync::mpsc;
use tokio::task;

use backend_warp::command::CommandToBackend;

use backend_warp::admin;
use backend_warp::events;
use backend_warp::expulsions;
use backend_warp::feedings;
use backend_warp::persons;
use backend_warp::static_file_filters;

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
    let in_thread_server = task::LocalSet::new();
    let (tx, rx) = mpsc::channel::<CommandToBackend>(32);

    let routes = persons::filters::all_persons(tx.clone())
        .or(feedings::filters::all_feedings(tx.clone()))
        .or(expulsions::filters::all_expulsions(tx.clone()))
        .or(events::filters::all_events(tx.clone()))
        .or(admin::filters::all_admin(tx.clone()))
        .or(static_file_filters::get_index())
        .or(static_file_filters::get_static_file())
        .or(static_file_filters::serve_index_by_default_get());

    let warp_server = tokio::spawn(async move {
        warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
    });

    let ost_file_path =
        env::var("OST_CONTEXT_FILE_PATH").expect("Missing env var: `OST_CONTEXT_FILE_PATH`");

    log::info!("Trying to load: {}", &ost_file_path);

    in_thread_server
        .run_until(async move {
            backend_warp::local_state::file_based_ost_context(rx, &ost_file_path).await
        })
        .await;

    warp_server.await.unwrap();
}
