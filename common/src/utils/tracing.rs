use tracing_subscriber::EnvFilter;

pub fn init_tracing(directives: Vec<&str>) {
    let mut env_filter = EnvFilter::from_default_env();
    for directive in directives {
        env_filter = env_filter.add_directive(directive.parse().unwrap());
    }
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_env_filter(env_filter)
        .init();
}
