use std::fmt;

use actix_web::{error, http::StatusCode, HttpResponse};
// 错误信息包装
use serde::Serialize;
#[derive(Debug, Serialize)]
pub enum MyError {
    // DBError(String),      // 数据库错误, 暂时不需要
    ActixError(String),   // 服务器错误
    NotFound(String),     // 资源未找到
    InvalidInput(String), // 前端非法传递
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    // 传递给用户的响应错误信息
    error_message: String,
}

// 将 MyError转换为 MyErrorResponse
impl MyError {
    fn error_response(&self) -> String {
        match self {
            // MyError::DBError(msg) => {
            //     println!("Database error occurred: {:?}", msg); // 打印错误信息
            //     "Database error".into()
            // }
            MyError::ActixError(msg) => {
                // 服务器错误
                println!("Server error occurred: {:?}", msg);
                // into 本质上是在执行对应类型的 from, 这里相当于执行 String::from("Internal Server error")
                "Internal Server error".into()
            }
            MyError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
            MyError::InvalidInput(msg) => {
                println!("Invalid parameters received: {:?}", msg);
                msg.into()
            }
        }
    }
}

// ResponseError 这个trait 就两个方法, 一个是 status_code, 另一个是 error_response
impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        // 只要不是 not Found, 均返回500
        match self {
            MyError::ActixError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_msg) => StatusCode::NOT_FOUND,
            MyError::InvalidInput(_msg) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        // build传入StatusCode, 返回一个 HttpResponse, 然后继续调用json形成一段自定义错误
        // ? . 操作符自动引用, 可以省略 &
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

// 实现 fmt::Display
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

// 转换错误信息
impl From<actix_web::error::Error> for MyError {
    // Self表示返回本身, 而不是实例
    fn from(err: actix_web::error::Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}
