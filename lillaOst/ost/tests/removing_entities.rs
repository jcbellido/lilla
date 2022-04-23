use ost::context::construct_monolith;

#[test]
fn feeds_can_be_removed() {
    let mono_file: &str = "./test_output/removing_entities_00.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        let zardoz = monolith_context.add_person("Zardoz").unwrap();
        let _ = monolith_context.add_feeding(&zardoz, 1, 1, 1).unwrap();
        let _ = monolith_context.add_feeding(&zardoz, 2, 2, 2).unwrap();
        let _ = monolith_context.add_feeding(&zardoz, 3, 3, 3).unwrap();
        let feeds = monolith_context.feedings();
        assert!(!feeds.is_empty());
        assert_eq!(feeds.len(), 3);
    }
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        let mut feeds = monolith_context.feedings();

        let feed_to_remove = feeds.remove(
            feeds
                .iter()
                .position(|feed| feed.breast_milk() == 2)
                .unwrap(),
        );

        let result = monolith_context.remove_feeding(feed_to_remove);
        assert!(result.is_ok());
    }
    {
        let monolith_context = construct_monolith(mono_file).unwrap();
        let feeds = monolith_context.feedings();
        assert!(!feeds.is_empty());
        assert_eq!(feeds.len(), 2);
        let res = feeds.iter().find(|feed| feed.breast_milk() == 2);
        assert!(res.is_none());
    }
    let _ignore_fail = std::fs::remove_file(mono_file);
}

#[test]
fn expulsions_can_be_removed() {
    let mono_file: &str = "./test_output/removing_entities_01.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        let zardoz = monolith_context.add_person("Zardoz").unwrap();
        let _ = monolith_context.add_expulsion(&zardoz, ost::expulsion::ExpulsionDegree::Clean);
        let _ = monolith_context.add_expulsion(&zardoz, ost::expulsion::ExpulsionDegree::Pee);
        let _ = monolith_context.add_expulsion(&zardoz, ost::expulsion::ExpulsionDegree::Shart);
        let feeds = monolith_context.expulsions();
        assert!(!feeds.is_empty());
        assert_eq!(feeds.len(), 3);
    }
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        let mut expulsions = monolith_context.expulsions();

        let expulsion_to_remove = expulsions.remove(
            expulsions
                .iter()
                .position(|expulsion| expulsion.degree() == ost::expulsion::ExpulsionDegree::Pee)
                .unwrap(),
        );

        let result = monolith_context.remove_expulsion(expulsion_to_remove);
        assert!(result.is_ok());
    }
    {
        let monolith_context = construct_monolith(mono_file).unwrap();
        let expulsions = monolith_context.expulsions();
        assert!(!expulsions.is_empty());
        assert_eq!(expulsions.len(), 2);
        let res = expulsions
            .iter()
            .find(|exp| exp.degree() == ost::expulsion::ExpulsionDegree::Pee);
        assert!(res.is_none());
    }

    let _ignore_fail = std::fs::remove_file(mono_file);
}

#[test]
fn context_can_be_purged() {
    let mono_file: &str = "./test_output/removing_entities_02.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        let zardoz = monolith_context.add_person("Zardoz").unwrap();
        let _ = monolith_context.add_expulsion(&zardoz, ost::expulsion::ExpulsionDegree::Clean);
        let _ = monolith_context.add_expulsion(&zardoz, ost::expulsion::ExpulsionDegree::Pee);
        let _ = monolith_context.add_expulsion(&zardoz, ost::expulsion::ExpulsionDegree::Shart);
        let feeds = monolith_context.expulsions();
        assert!(!feeds.is_empty());
        assert_eq!(feeds.len(), 3);
    }
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        let result = monolith_context.purge_all_data();
        assert!(result.is_ok());
    }
    {
        let monolith_context = construct_monolith(mono_file).unwrap();
        assert!(monolith_context.persons().is_empty());
        assert!(monolith_context.expulsions().is_empty());
    }

    let _ignore_fail = std::fs::remove_file(mono_file);
}

#[test]
fn context_events_can_be_purged() {
    let mono_file: &str = "./test_output/removing_entities_03.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        let zardoz = monolith_context.add_person("Zardoz").unwrap();
        let _ = monolith_context.add_expulsion(&zardoz, ost::expulsion::ExpulsionDegree::Clean);
        let _ = monolith_context.add_expulsion(&zardoz, ost::expulsion::ExpulsionDegree::Pee);
        let _ = monolith_context.add_expulsion(&zardoz, ost::expulsion::ExpulsionDegree::Shart);
        let feeds = monolith_context.expulsions();
        assert!(!feeds.is_empty());
        assert_eq!(feeds.len(), 3);
    }
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        let result = monolith_context.purge_all_events();
        assert!(result.is_ok());
    }
    {
        let monolith_context = construct_monolith(mono_file).unwrap();
        assert_eq!(monolith_context.persons().len(), 1);
        assert!(monolith_context.expulsions().is_empty());
    }

    let _ignore_fail = std::fs::remove_file(mono_file);
}
