use poem_openapi::Object;
use serde::{Deserialize, Serialize};

///  登录
#[derive(Debug, Serialize, Deserialize, Object)]
pub struct LoginReq {
    ///  用户名
    pub user: String,
    ///  用户密码
    pub password: String,
    ///验证码
    pub code: String,
    ///验证码唯一id
    pub uuid: String,
}

///  登录结果 //, sea_orm::FromQueryResult
#[derive(Debug, Serialize, Deserialize, Object)]
pub struct LoginRes {
    /// 令牌
    pub token: String,
    pub token_type: String,
    /// 有效期
    pub exp: i64,
    // 过期时间
    pub exp_in: i64,
}
