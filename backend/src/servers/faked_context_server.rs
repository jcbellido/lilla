use tokio::sync::mpsc;
use tokio::task;

use warp::Filter;

use crate::command::CommandToBackend;

use crate::admin;
use crate::events;
use crate::expulsions;
use crate::feedings;
use crate::persons;
use crate::static_file_filters;

#[allow(dead_code)]
pub async fn faked_context_server() {
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
        warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    });

    log::info!(">>> FAKE DATA ready");

    in_thread_server
        .run_until(
            async move { crate::local_state_fake_in_memory::faked_state_ost_context(rx).await },
        )
        .await;

    warp_server.await.unwrap();
}
