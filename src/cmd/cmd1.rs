use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;
use poem::middleware::Tracing;
use poem::{
    endpoint::{EmbeddedFileEndpoint, EmbeddedFilesEndpoint},
    listener::TcpListener,
    EndpointExt, Route, Server,
};
use rust_embed::RustEmbed;

use crate::api;
use crate::config;

#[derive(RustEmbed)]
#[folder = "web/dist"]
struct WebFiles;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, value_name = "FILE", default_value = "conf.toml")]
    config: PathBuf,

    /// Sets a port you want to listen
    #[arg(short, long, default_value_t = 3000)]
    #[arg(value_parser = clap::value_parser ! (u16).range(1..))]
    port: u16,
}

#[allow(dead_code)]
pub async fn run() -> Result<(), std::io::Error> {
    //命令行解析
    let cli = Cli::parse();

    println!("Config: {:?}", cli.config);
    println!("Port: {:?}", cli.port);

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }

    tracing_subscriber::fmt::init();
    println!("--> {:#?}", config::CFG.log);

    let app = Route::new()
        .at("/", EmbeddedFileEndpoint::<WebFiles>::new("index.html"))
        .nest("/", EmbeddedFilesEndpoint::<WebFiles>::new())
        .nest("/api", api::load_api())
        .with(Tracing);
    Server::new(TcpListener::bind(format!("0.0.0.0:{}", cli.port)))
        .run_with_graceful_shutdown(
            app,
            async move {
                let _ = tokio::signal::ctrl_c().await;
            },
            Some(Duration::from_secs(3)),
        )
        .await
}
