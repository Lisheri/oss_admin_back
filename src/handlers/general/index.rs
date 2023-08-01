use crate::modules::state::AppState;
use actix_web::{web, HttpResponse};

// web服务检查
// app_state 注入的 AppState
pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    // 防止 visit_count 被其他线程访问, 因此需要上锁, lock方法返回Result枚举, 使用 unwrap 简单处理
    let mut visit_count = app_state.visit_count.lock().unwrap();

    let response = format!("{} {} times", health_check_response, visit_count);
    // 访问数 + 1;
    *visit_count += 1;
    // 返回响应
    HttpResponse::Ok().json(&response)
}
