use ost::context::construct_monolith;

#[test]
fn person_changes_active() {
    let mono_file: &str = "./test_output/person_changes_00.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        let mut zardoz = monolith_context
            .add_person("Zardoz")
            .expect("adding a person just works");

        zardoz.set_is_active(false);
        assert!(!zardoz.is_active());
        let zardoz_modified = monolith_context.modify_person(&zardoz);
        assert!(zardoz_modified.is_ok());
    }
    {
        let monolith_context = construct_monolith(mono_file).unwrap();
        let persons = monolith_context.persons();
        assert_eq!(persons.len(), 1, "no added persons in storage");
        let retrieved_paco = persons.first().unwrap();
        assert_eq!("Zardoz", retrieved_paco.name());
        assert!(!retrieved_paco.is_active());
    }
    let _ignore_fail = std::fs::remove_file(mono_file);
}

#[test]
fn double_addition_of_person_fails() {
    let mono_file: &str = "./test_output/person_changes_01.json";
    let _ignore_fail = std::fs::remove_file(mono_file);

    let mut monolith_context = construct_monolith(mono_file).unwrap();
    let _zardoz = monolith_context
        .add_person("Zardoz")
        .expect("adding a person just works");

    let other_zardoz = monolith_context.add_person("Zardoz");
    assert!(other_zardoz.is_err());

    let _ignore_fail = std::fs::remove_file(mono_file);
}

#[test]
fn person_changes_propagate_to_feedings() {
    let mono_file: &str = "./test_output/person_changes_02.json";
    let _ignore_fail = std::fs::remove_file(mono_file);

    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        let mut zardoz = monolith_context.add_person("Zardoz").unwrap();
        let _ = monolith_context
            .add_feeding(&zardoz, 11, 22, 33)
            .expect("feed insertion works");

        zardoz.set_name("nu_zardoz");
        zardoz.set_is_active(false);
        monolith_context
            .modify_person(&zardoz)
            .expect("person modification should work");

        let feeds = monolith_context.feedings();
        assert_eq!(feeds.len(), 1);
        let nu_zardoz_feeding = feeds.first().unwrap();

        assert_eq!(nu_zardoz_feeding.person_name(), zardoz.name());
    }

    let _ignore_fail = std::fs::remove_file(mono_file);
}
