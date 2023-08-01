use actix_web::web;

use crate::handlers::oss::get_all_list;

pub fn oss_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/oss").route("/list", 
        web::get().to(get_all_list))
    );
}
