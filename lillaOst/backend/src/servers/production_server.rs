use std::env;

use tokio::sync::mpsc;
use tokio::task;

use crate::command::CommandToBackend;

use warp::Filter;

use crate::admin;
use crate::events;
use crate::expulsions;
use crate::feedings;
use crate::persons;
use crate::static_file_filters;

#[allow(dead_code)]
pub async fn production_server() {
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
        .run_until(
            async move { crate::local_state::file_based_ost_context(rx, &ost_file_path).await },
        )
        .await;

    warp_server.await.unwrap();
}
