use lillatelly::source_target_configuration::SourceTargetConfiguration;
use lillatelly::task_tv_show::TaskTvShow;

#[test]
fn list_source_files() {
    let source = r#"[
        {
            "source" : "./test_data/source_a",
            "target" : "does not really matter much"
        }
    ]"#;
    let mut all_configurations =
        serde_json::from_str::<Vec<SourceTargetConfiguration>>(source).unwrap();
    let _task = TaskTvShow {
        configuration: all_configurations.pop().unwrap(),
    };
}
