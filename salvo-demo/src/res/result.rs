use serde::{Deserialize, Serialize};

// 定义响应数据结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseData<T> {
    pub code: i8,
    pub message: String,
    pub data: Option<T>,
}

// 定义统一响应代码
pub const SUCCESS_CODE: i8 = 0;
pub const ERROR_CODE: i8 = -1;

// 统一响应结构体的实现
impl<T> ResponseData<T> {
    pub fn success(data: T, message: &str) -> Self {
        ResponseData {
            code: SUCCESS_CODE,
            message: message.to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: &str) -> Self {
        ResponseData {
            code: ERROR_CODE,
            message: message.to_string(),
            data: None,
        }
    }
}
