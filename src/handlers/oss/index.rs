use std::collections::HashMap;

use actix_web::{web, HttpResponse};
use oss_rust_sdk::prelude::ListObjects;
// use oss_sdk::bucket::ListBuckets;
use serde::Serialize;

use crate::{error::MyError, modules::state::AppState};

#[path = "./list_query.rs"]
mod list_query;

use list_query::ListQuery;

#[derive(Debug, Serialize)]
struct ResponseResult<'a, T> {
    code: u32,
    msg: &'a str,
    data: T,
}

pub async fn get_all_list(
    app_state: web::Data<AppState>,
    params: web::Query<ListQuery>,
) -> Result<HttpResponse, MyError> {
    let headers: HashMap<String, String> = HashMap::new();
    let mut resources: HashMap<String, Option<String>> = HashMap::new();
    let inner_params = params.into_inner();
    resources.insert("prefix".into(), Some(inner_params.prefix));
    resources.insert("max_keys".into(), Some(inner_params.max_keys));

    app_state
        .oss_client
        .clone()
        .get_list(headers, resources)
        .map(|list| {
            println!("list is: {:?}", &list);
            HttpResponse::Ok().json(ResponseResult::<ListObjects> {
                code: 0,
                msg: "success",
                data: list,
            })
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::{client::OssClient, state::AppState};
    use actix_web::http::StatusCode;
    use list_query::ListQuery;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_list_test() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            oss_client: OssClient::new(),
        });

        let params: web::Query<ListQuery> =
            web::Query::<ListQuery>::from_query("prefix=image&max_keys=10").unwrap();
        let res = get_all_list(app_state, params).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }
}
