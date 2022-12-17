use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;
use poem::middleware::Tracing;
use poem::web::RealIp;
use poem::{
    endpoint::{EmbeddedFileEndpoint, EmbeddedFilesEndpoint},
    listener::TcpListener,
    EndpointExt, FromRequest, Request, RequestBody, Route, Server,
};
use poem_openapi::__private::serde_json;
use poem_openapi::{
    param::Query, payload::Json, payload::PlainText, Object, OpenApi, OpenApiService,
};
use rust_embed::RustEmbed;

use crate::api;
use crate::config;
use crate::controller;

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
        .nest("/go", api::load_api())
        .nest("/api", load_openapi())
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

fn load_openapi() -> Route {
    let api_service =
        OpenApiService::new((controller::auth::Api, TestApi), "平台开放API说明", "1.0")
            .server("/api");
    let ui = api_service.swagger_ui();
    let ui2 = api_service.rapidoc();
    let ui3 = api_service.redoc();
    Route::new()
        .nest("/", api_service)
        .nest("/ui", ui)
        .nest("/ui2", ui2)
        .nest("/ui3", ui3)
}

struct TestApi;

#[OpenApi]
impl TestApi {
    #[oai(path = "/http", method = "get")]
    async fn patch(&self, req: HttpRes) -> Json<HttpRes> {
        Json(req)
    }

    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }
}

#[derive(Debug, Clone, Object)]
struct HttpRes {
    method: String,
    headers: HashMap<String, String>,
    uri: String,
    origin: Option<String>,
    query: HashMap<String, String>,
    body: Option<String>,
    json: Option<serde_json::Value>,
    form: Option<HashMap<String, String>>,
}

#[poem::async_trait]
impl<'a> FromRequest<'a> for HttpRes {
    async fn from_request(req: &'a Request, body: &mut RequestBody) -> Result<Self, poem::Error> {
        let method = req.method().to_string();

        let headers = req
            .headers()
            .iter()
            .map(|(header_name, header_value)| {
                (
                    header_name.to_string(),
                    header_value.to_str().unwrap().to_string(),
                )
            })
            .collect();

        let uri = req.uri().to_string();

        let origin = RealIp::from_request_without_body(req).await;
        let origin = origin
            .ok()
            .and_then(|real_ip| real_ip.0.map(|ip| ip.to_string()));

        let mut query = HashMap::new();
        let query_str = req.uri().query().unwrap_or("");
        query_str.split('&').for_each(|key_value| {
            let pair = key_value.split('=').collect::<Vec<_>>();
            if pair.len() > 1 {
                query.insert(pair[0].to_string(), pair[1].to_string());
            }
        });

        let body = body.take().unwrap().into_vec().await.ok();
        let body = body.map(|body| match String::from_utf8(body.clone()) {
            Ok(body) => body,
            Err(_) => "data:application/octet-stream;base64,".to_string() + &base64::encode(body),
        });

        let json = if req.content_type() == Some("application/json") {
            body.as_ref()
                .and_then(|body| serde_json::from_str(body).ok())
        } else {
            None
        };

        let form = if req.content_type() == Some("application/x-www-form-urlencoded") {
            body.as_ref()
                .and_then(|body| serde_urlencoded::from_str(body).ok())
        } else {
            None
        };

        Ok(Self {
            method,
            headers,
            uri,
            origin,
            query,
            body,
            json,
            form,
        })
    }
}
