// use ost::context::construct_string_based_monolith;

// #[test]
// fn can_construct() {
//     let file_source: &str = "./test_data/backend_events.json";
//     let content = std::fs::read_to_string(file_source).expect("file exist and can be read");

//     assert!(!content.is_empty());

//     let string_based_context = construct_string_based_monolith(content);
//     assert!(string_based_context.is_ok());

//     let mono = string_based_context.unwrap();
//     assert_eq!(mono.persons().len(), 3);
//     assert_eq!(mono.feedings().len(), 300);
// }

// fn external_persist(payload: String) {
//     assert!(!payload.is_empty());
// }

// #[test]
// fn persist_callback() {
//     let file_source: &str = "./test_data/backend_events.json";
//     let content = std::fs::read_to_string(file_source).expect("file exist and can be read");
//     let _ = construct_string_based_monolith(content, external_persist).unwrap();
// }
