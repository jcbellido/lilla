use tokio::sync::mpsc;
use tokio::task;
use warp::http::StatusCode;
use warp::test::request;

use backend::command;
use backend::feedings;
use backend::local_state_fake_in_memory::faked_state_ost_context;

#[tokio::test]
async fn ost_get_feedings_00() {
    let in_thread_server = task::LocalSet::new();
    let (tx, rx) = mpsc::channel::<command::CommandToBackend>(32);

    let request = tokio::spawn(async move {
        let f_ost_get_feedings = feedings::filters::get_feedings(tx.clone()).clone();
        let response = request()
            .method("GET")
            .path("/api/feedings")
            .reply(&f_ost_get_feedings)
            .await;
        assert_eq!(response.status(), StatusCode::OK);

        let payload = response.body().to_vec();
        let message = std::str::from_utf8(&payload).unwrap().to_string();

        assert!(!message.is_empty());
    });

    in_thread_server
        .run_until(async move { faked_state_ost_context(rx).await })
        .await;
    request.await.unwrap();
}
