use poem_openapi::payload::Json;
use poem_openapi::OpenApi;

use crate::api::auth::{LoginReq, LoginRes};
use crate::api::res::{ApiResult, Resp};

/// # 处理登录、授权api
pub struct Api;

#[OpenApi]
impl Api {
    /// 登录
    #[oai(path = "/login", method = "post")]
    async fn login(&self, login: Json<LoginReq>) -> ApiResult<LoginRes> {
        let v = LoginRes {
            token: format!(
                "Berera aadfdfdfsfdfdfdsfs {}:{}",
                login.user, login.password
            ),
            token_type: "Berera".to_string(),
            exp: 222,
            exp_in: 43440,
        };
        Resp::with_data(v)
    }

    /// 登出
    #[oai(path = "/logout", method = "get")]
    async fn logout(&self) -> ApiResult<String> {
        Resp::with_msg("logout")
    }
}
