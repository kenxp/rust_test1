use rbatis::rbdc::datetime::{DateTime, FastDateTime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct sys_user {
    pub id: String,
    pub user_name: String,
    pub user_nickname: String,
    pub user_password: String,
    pub user_salt: String,
    pub user_status: String,
    pub user_email: Option<String>,
    pub sex: String,
    pub avatar: String,
    pub role_id: String,
    pub dept_id: String,
    pub remark: Option<String>,
    pub is_admin: String,
    pub phone_num: Option<String>,
    pub last_login_ip: Option<String>,
    pub last_login_time: Option<FastDateTime>,
    pub created_at: FastDateTime,
    pub updated_at: Option<FastDateTime>,
    pub deleted_at: Option<FastDateTime>,
}

///Background user table
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct sys_user2 {
    pub id: String,
    pub account: String,
    pub password: String,
    pub salt: String,
    pub name: Option<String>,
    pub status: Option<i32>,
    pub email: Option<String>,
    pub sex: String,
    pub avatar: String,
    pub role_id: String,
    pub dept_id: String,
    pub vcode: Option<String>,
    pub is_admin: Option<i32>,
    pub phone: Option<String>,
    pub login_check: Option<i32>,
    pub last_login_ip: Option<String>,
    pub last_login_time: Option<FastDateTime>,
    pub created_at: Option<FastDateTime>,
    pub updated_at: Option<FastDateTime>,
    pub deleted_at: Option<FastDateTime>,
}
