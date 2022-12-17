use log::{info, warn};
use rbatis::{crud, impl_select_page, Rbatis};
//use rbdc_mssql::driver::MssqlDriver;
use rbdc_mysql::driver::MysqlDriver;
use rbdc_pg::driver::PgDriver;
use rbdc_sqlite::driver::SqliteDriver;

use crate::model::entity::*;

/// # 数据访问
// 数据访问对象，这是一层抽象对象，用于和底层数据库交互，仅包含最基础的 CURD 方法
// - dao层通过框架的ORM抽象层组件与底层真实的数据库交互。
// - dao层的代码应该尽量保证通用性，并且大部分场景下不需要增加额外方法，只需要使用一些通用的链式操作方法拼凑即可满足。业务逻辑、包括看似只是简单的数据操作的逻辑都应当封装到service中，service中包含多个业务模块，每个模块独自管理自己的dao对象，service与service之间通过相互调用方法来实现数据通信而不是随意去调用其他service模块的dao对象。

/// 初始化数据库连接
pub fn init_db(db_source: &str) -> Rbatis {
    let rb = Rbatis::new();
    if rb.is_debug_mode() == true {
        warn!(r#"Rbatis正在使用debug模式！Release请修改Cargo.toml 中debug配置项为  debug: false"#);
    }
    /// connect to database
    //rb.init(SqliteDriver {}, "sqlite://data/sqlite.db").unwrap();
    // mysql
    //rb.init(MysqlDriver {}, "mysql://root:abcd1234@localhost:3306/rust_test").unwrap();
    // postgresql
    rb.init(
        PgDriver {},
        "postgres://root:abcd1234@localhost:5432/postgres",
    )
    .unwrap();

    info!(
        "pool status: {:?}",
        rb.get_pool().expect("pool not init!").status()
    );

    return rb;
}

crud!(sys_user {});
impl_select_page!(sys_user{select_page(name:&str,account:&str)=>
    "`where 1=1`
        if name != '':
          ` and user_name like #{'%'+name+'%'}`
        if account != '':
          ` and user_nickname like #{'%'+account+'%'}`
        if !sql.contains('count'):
         ` order by created_at desc`"});
