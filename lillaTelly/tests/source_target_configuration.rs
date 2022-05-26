use lillatelly::source_target_configuration::SourceTargetConfiguration;

#[test]
fn can_construct_a_trivial_configuration() {
    let source = r#"[
        {
            "source" : "/sfoo/sbar/saux",
            "target" : "/dfoo/dbar/daux",
            "default_season" : 30,
            "default_episode": 1
        }
    ]"#;

    let deserialized = serde_json::from_str::<Vec<SourceTargetConfiguration>>(source)
        .expect("Structure should be deserializable");

    assert_eq!(deserialized[0].source, "/sfoo/sbar/saux");
    assert_eq!(deserialized[0].target, "/dfoo/dbar/daux");
}
