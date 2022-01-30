use tokio::sync::mpsc;
use tokio::task;
use warp::http::StatusCode;
use warp::test::request;

use backend_warp::admin;
use backend_warp::command;
use backend_warp::command_args::ArgAddPerson;
use backend_warp::local_state::faked_state_ost_context;
use backend_warp::persons;
use ost::person::{deserialize as person_deserialize, Person};

#[tokio::test]
async fn can_reset_remote_context() {
    let in_thread_server = task::LocalSet::new();
    let (tx, rx) = mpsc::channel::<command::CommandToBackend>(32);

    let tx_add = tx.clone();

    let arg = ArgAddPerson {
        name: "manolo".to_string(),
    };

    let request_add_person = tokio::spawn(async move {
        let f_add_person = persons::filters::add_person(tx_add.clone()).clone();
        let response_add_person = request()
            .method("POST")
            .path("/api/persons")
            .json(&arg)
            .reply(&f_add_person)
            .await;

        assert_eq!(response_add_person.status(), StatusCode::OK);
    });

    let tx_reset = tx.clone();

    let request_context_reset = tokio::spawn(async move {
        let f_context_reset = admin::filters::reset(tx_reset.clone()).clone();
        let response_context_reset = request()
            .method("POST")
            .path("/api/admin/reset")
            .reply(&f_context_reset)
            .await;

        assert_eq!(response_context_reset.status(), StatusCode::OK);
    });

    let request_get_persons = tokio::spawn(async move {
        let f_ost_get_persons = persons::filters::get_persons(tx.clone()).clone();
        let response = request()
            .method("GET")
            .path("/api/persons")
            .reply(&f_ost_get_persons)
            .await;
        assert_eq!(response.status(), StatusCode::OK);

        let payload = response.body().to_vec();
        let message = std::str::from_utf8(&payload).unwrap().to_string();

        let vec_of_serialized_persons: Vec<String> = serde_json::from_str(&message).unwrap();

        #[allow(clippy::needless_collect)]
        let reconstructed: Vec<Box<dyn Person>> = vec_of_serialized_persons
            .iter()
            .map(|s| person_deserialize(s).unwrap())
            .collect();
        assert_eq!(reconstructed.len(), 10);
    });

    in_thread_server
        .run_until(async move { faked_state_ost_context(rx).await })
        .await;

    request_add_person.await.unwrap();
    request_context_reset.await.unwrap();
    request_get_persons.await.unwrap();
}
