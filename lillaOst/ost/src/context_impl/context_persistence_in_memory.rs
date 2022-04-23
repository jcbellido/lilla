use super::context_monolithic_impl::ContextMonolithicImpl;

pub fn new_monolith() -> Result<ContextMonolithicImpl, String> {
    let nu_monolith = ContextMonolithicImpl {
        target_file: "executing in memory".to_string(),
        persons: vec![],
        feeds: vec![],
        expulsions: vec![],
        events: vec![],
        persist_function: persist,
    };
    Ok(nu_monolith)
}

pub fn persist(_: &ContextMonolithicImpl) -> Result<(), String> {
    Ok(())
}
