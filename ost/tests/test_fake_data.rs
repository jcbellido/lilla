use ost::context::construct_monolith;

#[test]
fn can_generate_fake_persons() {
    let mono_file: &str = "./test_output/fake_data_00.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        let result = monolith_context.add_fake_persons(5);
        assert!(result.is_ok());
        assert_eq!(monolith_context.persons().len(), 5);
    }
    let _ignore_fail = std::fs::remove_file(mono_file);
}

#[test]
fn can_generate_fake_feedings() {
    let mono_file: &str = "./test_output/fake_data_01.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        assert!(monolith_context.add_fake_persons(5).is_ok());
        assert!(monolith_context.add_fake_feedings(30).is_ok());
        assert_eq!(monolith_context.feedings().len(), 30);
    }
    let _ignore_fail = std::fs::remove_file(mono_file);
}

#[test]
fn can_generate_fake_expulsions() {
    let mono_file: &str = "./test_output/fake_data_02.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        assert!(monolith_context.add_fake_persons(5).is_ok());
        assert!(monolith_context.add_fake_expulsions(66).is_ok());
        assert_eq!(monolith_context.expulsions().len(), 66);
    }
    let _ignore_fail = std::fs::remove_file(mono_file);
}

#[test]
fn can_generate_fake_events() {
    let mono_file: &str = "./test_output/fake_data_03.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        assert!(monolith_context.add_fake_persons(5).is_ok());
        assert!(monolith_context.add_fake_events(77).is_ok());
        assert_eq!(monolith_context.events().len(), 77);
    }
    let _ignore_fail = std::fs::remove_file(mono_file);
}

#[test]
fn generate_events_for_backend() {
    let mono_file: &str = "./test_output/backend_events.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        assert!(monolith_context.add_fake_persons(3).is_ok());
        assert!(monolith_context.add_fake_events(150).is_ok());
        assert!(monolith_context.add_fake_expulsions(600).is_ok());
        assert!(monolith_context.add_fake_feedings(300).is_ok());
    }
}
