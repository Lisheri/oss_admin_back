use crate::handlers::general::health_check_handler;
use actix_web::web::{ ServiceConfig, get };

pub fn general_routes(cfg: &mut ServiceConfig) {
    cfg.route("/health", get().to(health_check_handler));
}
