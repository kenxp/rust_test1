use std::fs::File;

use rbatis::rbdc::datetime::DateTime;
use rbatis::Rbatis;
use rbdc_sqlite::driver::SqliteDriver;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BizActivity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<DateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}

async fn test_sqlite() {
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");

    // let f = File::create("./target/test.db");
    // if f.is_err() {
    //     println!("{}", f.err().unwrap());
    // } else {
    //     drop(f);
    // }
    let rb = Rbatis::new();
    rb.init(SqliteDriver {}, "sqlite://:memory:").unwrap();

    rb.exec(
        "CREATE TABLE `biz_activity`
(
    `id`            TEXT PRIMARY KEY NOT NULL,
    `name`          TEXT     DEFAULT NULL,
    `pc_link`       TEXT     DEFAULT NULL,
    `h5_link`       TEXT     DEFAULT NULL,
    `sort`          TEXT     DEFAULT NULL,
    `status`        INT      DEFAULT NULL,
    `version`       INT      DEFAULT NULL,
    `remark`        TEXT     DEFAULT NULL,
    `create_time`   datetime DEFAULT NULL,
    `delete_flag`   INT(1)   DEFAULT NULL,
    `pc_banner_img` TEXT     DEFAULT NULL,
    `h5_banner_img` TEXT     DEFAULT NULL
);

INSERT INTO `biz_activity`
VALUES ('1', '活动1', NULL, NULL, '1', 1, 1, '', '2019-12-12 00:00:00', 0, NULL, NULL),
       ('223', 'test', '', '', '0', 0, 0, '', '2020-06-17 20:10:23', 0, NULL, NULL);",
        vec![],
    )
    .await
    .expect("TODO: panic message");

    let data: Vec<BizActivity> = rb
        .fetch_decode("select * from biz_activity", vec![])
        .await
        .unwrap();
    for mut x in data {
        println!("row: {:?}", x);
    }
}

pub async fn run() -> () {
    test_sqlite().await
}
