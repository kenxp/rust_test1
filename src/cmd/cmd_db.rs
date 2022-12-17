use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;
use poem::middleware::Tracing;
use poem::{
    endpoint::{EmbeddedFileEndpoint, EmbeddedFilesEndpoint},
    listener::TcpListener,
    EndpointExt, Route, Server,
};
use rbatis::impl_select;
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::sql::PageRequest;
use rust_embed::RustEmbed;

use crate::api;
use crate::config;
use crate::dao;
use crate::model;
use crate::model::entity::sys_user;

#[allow(dead_code)]
pub async fn run() -> () {
    /// enable log crate to show sql logs
    fast_log::LOGGER.set_level(log::LevelFilter::Info);
    fast_log::init(fast_log::Config::new().console()).expect("logger init fail.");

    println!("start");

    let mut rb = dao::init_db("");

    let table: Option<sys_user> = rb
        .fetch_decode("select * from sys_user limit ?", vec![rbs::to_value!(1)])
        .await
        .unwrap();

    println!(">>>>> table={:?}", table);

    let all = sys_user::select_all(&mut rb).await.unwrap();
    println!(">>>>> all={:?}", all);

    let sql: Vec<crate::model::entity::sys_user> = rb
        .fetch_decode("select * from sys_user where user_nickname='神马'", vec![])
        .await
        .unwrap();
    println!(">>>>> sql={:?}", sql);

    let vo = &sql[0];
    println!(">>>>> vo={:?}  ", vo);
    println!(">>>>> dt={:?}  {}", vo.deleted_at, vo.deleted_at.is_some());

    let b = vo.deleted_at.as_ref();
    println!(">>>>> b={:?}  ", b);

    let raw = rb
        .fetch("select * from sys_user where user_nickname='神马'", vec![])
        .await
        .unwrap();
    println!(">>>>> raw={:?}", raw);

    let data = sys_user::select_page(&mut rb, &PageRequest::new(2, 2), "", "")
        .await
        .unwrap();
    println!("select_page = {:?}", data);

    let data = sys_user::select_by_column(&mut rb, "id", "1")
        .await
        .unwrap()
        .into_iter()
        .next();
    println!("select_id1 = {:?}", data);
    println!("done");
}
