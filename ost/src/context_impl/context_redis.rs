// extern crate redis;

// use crate::context::Context;
// use crate::entity_impl::person_impl::PersonImpl;
// use crate::person::Person;

// // https://docs.rs/redis/0.21.3/redis/
// struct RedisContext {
//     #[allow(dead_code)]
//     connection: redis::Connection,
// }

// impl RedisContext {
//     #[allow(dead_code)]
//     pub fn new(connection: &str) -> Result<Self, String> {
//         match redis::Client::open(connection) {
//             Ok(client) => match client.get_connection() {
//                 Ok(connection) => Ok(RedisContext { connection }),
//                 Err(err) => Err(err.to_string()),
//             },
//             Err(err) => Err(err.to_string()),
//         }
//     }
// }

// impl Context for RedisContext {
//     fn get_all_persons(&self) -> Vec<Box<dyn Person>> {
//         vec![]
//     }

//     fn insert_new_person(&mut self, _new_person: dyn Person) -> Result<(), String> {
//         todo!()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::entity_impl::person_impl::PersonImpl;

//     use super::RedisContext;
//     use crate::context::Context;

//     #[test]
//     #[ignore = "on hold: Redis is overkill"]
//     fn test_docker_connection() {
//         let redis_context = RedisContext::new("redis://localhost:6379/");
//         match redis_context {
//             Ok(_context_built) => {}
//             Err(err) => panic!("{}", err),
//         }
//     }

//     #[test]
//     #[ignore = "on hold: Redis is overkill"]
//     fn fetch_empty_persons() {
//         let persistence = RedisContext::new("redis://localhost:6379/").unwrap();
//         let persons = persistence.get_all_persons();
//         assert!(persons.is_empty(), "fetching no persons failed?");
//     }

//     #[test]
//     #[ignore = "on hold: Redis is overkill"]
//     fn add_person() {
//         let mut persistence = RedisContext::new("redis://localhost:6379/").unwrap();
//         let paco = PersonImpl::new("paco");
//         persistence
//             .insert_new_person(paco.clone())
//             .expect("redis persistence, is it failing?");
//         let persons = persistence.get_all_persons();
//         assert_eq!(persons.len(), 1, "no added persons in storage");
//         let retrieved_paco = persons.first().unwrap();
//         assert!(paco == *retrieved_paco);
//     }
// }
