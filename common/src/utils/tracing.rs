use tracing_appender::{non_blocking, rolling::daily};
use tracing_subscriber as ts;
use ts::{EnvFilter, fmt::layer as tracing_layer, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing(
    directives: Vec<&str>,
    log_file: bool,
) -> Option<tracing_appender::non_blocking::WorkerGuard> {
    let mut env_filter = EnvFilter::from_default_env();
    for directive in directives {
        env_filter = env_filter.add_directive(directive.parse().unwrap());
    }

    let fmt_layer = tracing_layer()
        .with_line_number(true)
        .with_ansi(true)
        .with_writer(std::io::stdout);

    let file_appender = daily("logs", "");
    let (nb, guard) = non_blocking(file_appender);

    if log_file {
        let file_layer = tracing_layer()
            .with_line_number(true)
            .with_ansi(false)
            .with_writer(nb);

        ts::registry()
            .with(env_filter)
            .with(fmt_layer)
            .with(file_layer)
            .init();
        Some(guard)
    } else {
        ts::registry().with(env_filter).with(fmt_layer).init();
        None
    }
}
