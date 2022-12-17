mod api;
mod cmd;
mod config;
mod consts;
mod controller;
mod dao;
mod logic;
mod model;
mod service;
mod util;

// #[tokio::main]
// async fn main() -> Result<(), std::io::Error> {
//     println!("{}", consts::TEST);
//     cmd::cmd_openapi::run().await
// }

#[tokio::main]
async fn main() {
    println!("{}", consts::TEST);
    //cmd::cmd_openapi::run().await
    //cmd::cmd_db::run().await;
    cmd::cmd_test::run().await;
}
