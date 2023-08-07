use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use routers::oss::oss_routes;
use std::{io, sync::Mutex};

#[path = "../modules/modules.rs"]
mod modules;

#[path = "../routers/routers.rs"]
mod routers;

#[path = "../handlers/handlers.rs"]
mod handlers;

#[path = "../error/error.rs"]
mod error;

use error::MyError;
use modules::{client::OssClient, state::AppState};

use crate::routers::health::general_routes;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // 创建可共享的 state
    let shared_data = web::Data::new(AppState {
        health_check_response: "非常好".to_string(),
        visit_count: Mutex::new(0), // 访问数量
        oss_client: OssClient::new(),
    });

    // 创建 应用服务器
    let app = move || {
        // 处理跨域
        let cors = Cors::default()
            .allowed_origin("http://localhost:8000")
            .allowed_origin_fn(|origin: &http::header::HeaderValue, _req_head| {
                // 允许所有以 localhost 开头的域
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST", "DELETE"]) // 允许的请求方法
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE) // 允许的请求头
            .max_age(3600); // 3600s未响应就截断
        App::new()
            // 注册全局状态管理state
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                // 注册拦截不合法请求, 如果检测到前端传递不合法输入, 就会进入
                MyError::InvalidInput("Please provide valid json input".to_string()).into()
            }))
            .configure(general_routes)
            .wrap(cors)
            .configure(oss_routes)
    };
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
