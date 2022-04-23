use ost::context::{construct_monolith, construct_monolith_from_file};

static MONOLITHEMPTY: &str = "./test_data/monolith_empty.json";
static MONOLITHFOUTPUTFILE00: &str = "./test_output/integration_monolith_00.json";

#[test]
fn test_construct_monolith() {
    let monolith_context = construct_monolith(MONOLITHEMPTY);
    assert!(monolith_context.is_ok());
}

#[test]
fn test_construct_monolith_from_file() {
    let _ignore_fail = std::fs::remove_file(MONOLITHFOUTPUTFILE00);
    let monolith_context = construct_monolith_from_file(MONOLITHEMPTY, MONOLITHFOUTPUTFILE00);
    assert!(monolith_context.is_ok());
    let _ignore_fail = std::fs::remove_file(MONOLITHFOUTPUTFILE00);
}

static MONOLITHFOUTPUTFILE01: &str = "./test_output/integration_monolith_01.json";
#[test]
fn test_can_insert_persons() {
    let _ignore_fail = std::fs::remove_file(MONOLITHFOUTPUTFILE01);
    {
        let mut monolith_context = construct_monolith(MONOLITHFOUTPUTFILE01).unwrap();
        monolith_context
            .add_person("paco")
            .expect("insert new person is supposed to just work");
    }
    {
        let monolith_context = construct_monolith(MONOLITHFOUTPUTFILE01).unwrap();
        let persons = monolith_context.persons();
        assert_eq!(persons.len(), 1, "no added persons in storage");
        let retrieved_paco = persons.first().unwrap();
        assert_eq!("paco", retrieved_paco.name());
    }
    let _ignore_fail = std::fs::remove_file(MONOLITHFOUTPUTFILE01);
}
