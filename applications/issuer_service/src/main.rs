mod cli;
mod handlers;
mod server;

use crate::cli::Cli;
use crate::handlers::HandlerContext;
use futures::FutureExt;
use log::error;
use stable_coin_store::SqliteStore;
use std::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logging();
    let cli = Cli::init();

    ensure_directories_exist(&cli)?;
    let store = SqliteStore::connect(cli.common.db_url()).await?;
    store.migrate().await?;

    let context = HandlerContext::new(store, cli.common.jwt_secret);
    server::listen(
        cli.common.server_listen_address,
        context,
        tokio::signal::ctrl_c().map(|r| {
            if let Err(e) = r {
                error!("Error during shutdown: {}", e);
            }
        }),
    )
    .await?;
    Ok(())
}

fn ensure_directories_exist(cli: &Cli) -> anyhow::Result<()> {
    let path = cli.common.db_path.parent().unwrap();
    fs::create_dir_all(path)?;
    Ok(())
}

fn setup_logging() {
    let _ignore = fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {} {}",
                humantime::format_rfc3339(std::time::SystemTime::now()),
                record.target(),
                record.level(),
                message
            ))
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Debug)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        // .chain(fern::log_file("output.log").unwrap())
        // Apply globally
        .apply();
}
