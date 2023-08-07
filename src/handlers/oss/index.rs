use std::{collections::HashMap, rc::Rc};

use actix_web::{web, HttpResponse};
use oss_rust_sdk::object;
// use oss_sdk::bucket::ListBuckets;
use serde::Serialize;

use crate::{
    error::MyError,
    modules::state::{AppState, SUCCESS_CODE},
};

#[path = "./list_query.rs"]
mod list_query;

#[path = "./utils.rs"]
mod utils;

use list_query::ListQuery;

use self::{
    list_query::{ListObjectCamelCase, PutFolderBody},
    utils::{
        get_is_end, get_last_marker, get_oss_params, transform_list_object_data_to_camel_case,
    },
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ResponseResult<'a, T> {
    code: &'static u32,
    msg: &'a str,
    data: T,
}

pub async fn get_all_list(
    app_state: web::Data<AppState>,
    params: web::Query<ListQuery>,
) -> Result<HttpResponse, MyError> {
    let headers: HashMap<String, String> = HashMap::new();
    let inner_params = params.into_inner();
    let resources = get_oss_params(&inner_params);

    app_state
        .oss_client
        .clone()
        .get_list(headers, resources)
        .map(|list| {
            let contents_ref: &Vec<object::Object> = list.contents();
            let common_prefix_ref: &Vec<object::CommonPrefix> = list.common_prefixes();
            // 记录最后一位marker, 返回给前端, 分页查询时候带上
            let last_marker: &str = get_last_marker(contents_ref, common_prefix_ref);
            let is_end = get_is_end(&inner_params, contents_ref.len() + common_prefix_ref.len());
            // 转换字段
            let data: ListObjectCamelCase =
                transform_list_object_data_to_camel_case(list.clone(), last_marker, is_end);
            HttpResponse::Ok().json(ResponseResult::<ListObjectCamelCase> {
                code: &SUCCESS_CODE,
                msg: "success",
                data,
            })
        })
}

// 新建文件夹
pub async fn put_file_folder(
    app_state: web::Data<AppState>,
    params: web::Json<PutFolderBody>,
) -> Result<HttpResponse, MyError> {
    let headers: HashMap<String, String> = HashMap::new();
    // let params_ref = Rc::new(params);
    let inner_params = params.into_inner();
    let resources = get_oss_params(&ListQuery::from(&inner_params));
    app_state
        .oss_client
        .clone()
        .put_file_folder(inner_params.folder_name.unwrap(), headers, resources)
        .map(|res| {
            HttpResponse::Ok().json(ResponseResult::<()> {
                code: &SUCCESS_CODE,
                msg: "success",
                data: res,
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
