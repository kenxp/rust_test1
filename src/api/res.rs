use std::fmt::Debug;

use poem::Error;
use poem_openapi::{
    payload::Json,
    types::{ParseFromJSON, ToJSON},
    Object,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
/// 查 数据返回
pub struct ListData<T> {
    pub list: Vec<T>,
    pub total: u64,
    pub total_pages: u64,
    pub page_num: u64,
}

/// 分页参数
#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct PageParams {
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}

/// 填入到extensions中的数据
#[derive(Debug)]
pub struct ResJsonString(pub String);

/// API返回结果
pub type ApiResult<T> = poem::Result<Json<Resp<T>>>;

/// # json统一返回格式
#[derive(Object, Deserialize, Serialize, Clone, Debug)]
pub struct Resp<T>
where
    T: ParseFromJSON + ToJSON + Serialize + Send + Sync,
{
    /// 返回码，正常为0
    pub code: i32,
    /// 信息说明
    pub msg: String,
    /// 数据
    pub data: Option<T>,
}

#[allow(dead_code)]
impl<T> Resp<T>
where
    T: ParseFromJSON + ToJSON + Serialize + Send + Sync,
{
    pub fn with_data(data: T) -> ApiResult<T> {
        Ok(Json(Resp {
            code: 0,
            msg: "ok".to_string(),
            data: Some(data),
        }))
    }
    pub fn with_err(err: &str) -> ApiResult<T> {
        Ok(Json(Resp {
            code: 2,
            msg: err.to_string(),
            data: None,
        }))
    }
    pub fn with_msg(msg: &str) -> ApiResult<T> {
        Ok(Json(Resp {
            code: 0,
            msg: msg.to_string(),
            data: None,
        }))
    }
    pub fn with_data_msg(data: T, msg: &str) -> ApiResult<T> {
        Ok(Json(Resp {
            code: 0,
            msg: msg.to_string(),
            data: Some(data),
        }))
    }

    pub fn err(error: Error) -> ApiResult<T> {
        Err(error.into())
    }
}
