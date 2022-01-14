use ost::context::construct_monolith;

#[test]
fn can_get_filtered_feeds() {
    let mono_file: &str = "./test_output/filter_results_00.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        monolith_context.add_fake_persons(1).unwrap();
        monolith_context.add_fake_feedings(30).unwrap();

        let zardoz = monolith_context.add_person("Zardoz").unwrap();
        monolith_context
            .add_feeding(&zardoz, 11, 22, 33)
            .expect("feed insertion works");
        monolith_context
            .add_feeding(&zardoz, 33, 44, 55)
            .expect("feed insertion works");
        let persons = monolith_context.persons();
        let p = persons.first().unwrap();
        assert_eq!(monolith_context.feedings_by(p).len(), 30);
    }
    let _ignore_fail = std::fs::remove_file(mono_file);
}

#[test]
fn can_get_filtered_expulsions() {
    let mono_file: &str = "./test_output/filter_results_01.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        monolith_context.add_fake_persons(1).unwrap();
        monolith_context.add_fake_expulsions(22).unwrap();
        let persons = monolith_context.persons();
        let p = persons.first().unwrap();

        let zardoz = monolith_context.add_person("Zardoz").unwrap();
        let _ = monolith_context.add_expulsion(&zardoz, ost::expulsion::ExpulsionDegree::Pee);
        let _ = monolith_context.add_expulsion(&zardoz, ost::expulsion::ExpulsionDegree::Poopies);

        assert_eq!(monolith_context.expulsions_by(p).len(), 22);
    }
    let _ignore_fail = std::fs::remove_file(mono_file);
}

#[test]
fn can_get_filtered_events() {
    let mono_file: &str = "./test_output/filter_results_02.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        monolith_context.add_fake_persons(1).unwrap();
        monolith_context.add_fake_events(33).unwrap();
        let persons = monolith_context.persons();
        let p = persons.first().unwrap();

        let zardoz = monolith_context.add_person("Zardoz").unwrap();
        let _ = monolith_context.add_event(&zardoz, ost::event::EventType::Awake);
        let _ = monolith_context.add_event(&zardoz, ost::event::EventType::Sleep);

        assert_eq!(monolith_context.events_by(p).len(), 33);
    }
    let _ignore_fail = std::fs::remove_file(mono_file);
}
