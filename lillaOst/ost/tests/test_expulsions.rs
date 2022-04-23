use chrono::prelude::*;
use chrono::Utc;

use ost::context::construct_monolith;

use ost::expulsion::ExpulsionDegree;

#[test]
fn on_context_construct_expulsions_are_empty() {
    let mono_file: &str = "./test_output/test_expulsions_00.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    let monolith_context = construct_monolith(mono_file).unwrap();
    let expulsions = monolith_context.expulsions();
    assert!(expulsions.is_empty());
    let _ignore_fail = std::fs::remove_file(mono_file);
}

#[test]
fn can_insert_expulsions() {
    let mono_file: &str = "./test_output/test_expulsions_01.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    let mut monolith_context = construct_monolith(mono_file).unwrap();
    let zardoz = monolith_context.add_person("Zardoz").unwrap();

    let _ = monolith_context.add_expulsion(&zardoz, ExpulsionDegree::Poopies);

    let expulsions = monolith_context.expulsions();
    assert!(!expulsions.is_empty());
    assert_eq!(expulsions.len(), 1);
    let _ignore_fail = std::fs::remove_file(mono_file);
}

#[test]
fn expulsions_persist() {
    let mono_file: &str = "./test_output/test_expulsions_02.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        let zardoz = monolith_context.add_person("Zardoz").unwrap();
        let _ = monolith_context.add_expulsion(&zardoz, ExpulsionDegree::Poopies);
    }
    {
        let monolith_context = construct_monolith(mono_file).unwrap();
        let expulsions = monolith_context.expulsions();
        assert!(!expulsions.is_empty());
        assert_eq!(expulsions.len(), 1);
    }
    let _ignore_fail = std::fs::remove_file(mono_file);
}

#[test]
fn modify_expulsion() {
    let mono_file: &str = "./test_output/test_expulsions_03.json";
    let _ignore_fail = std::fs::remove_file(mono_file);
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        let zardoz = monolith_context.add_person("Zardoz").unwrap();
        let _ = monolith_context.add_expulsion(&zardoz, ExpulsionDegree::Poopies);
    }
    {
        let mut monolith_context = construct_monolith(mono_file).unwrap();
        let mut expulsions = monolith_context.expulsions();
        assert!(!expulsions.is_empty());

        let exp = expulsions.first_mut().expect("expulsion should exist");

        let bd = Utc.ymd(1979, 6, 10).and_hms(0, 0, 0);

        exp.modify_expulsion(ExpulsionDegree::Pooplosion, bd);
        assert_eq!(exp.degree(), ExpulsionDegree::Pooplosion);

        let result = monolith_context.modify_expulsion(exp);
        assert!(result.is_ok());
    }
    {
        let monolith_context = construct_monolith(mono_file).unwrap();
        let expulsions = monolith_context.expulsions();
        let expulsion = expulsions
            .iter()
            .find(|exp| exp.degree() == ExpulsionDegree::Pooplosion);
        assert!(expulsion.is_some());

        let expulsion = expulsion.unwrap();

        assert_eq!(
            expulsion.time_stamp().clone(),
            Utc.ymd(1979, 6, 10).and_hms(0, 0, 0)
        );
    }
    let _ignore_fail = std::fs::remove_file(mono_file);
}
