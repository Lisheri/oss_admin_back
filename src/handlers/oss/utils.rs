use std::{collections::HashMap, env};

use oss_rust_sdk::{object, prelude::ListObjects};

use super::list_query::{CommonPrefix, Content, ListObjectCamelCase, ListQuery, Owner};
use dotenv::dotenv;

pub fn get_oss_params(inner_params: &ListQuery) -> HashMap<String, Option<String>> {
    let mut resources: HashMap<String, Option<String>> = HashMap::new();
    resources.insert("prefix".into(), inner_params.prefix.clone());
    resources.insert("max-keys".into(), inner_params.max_keys.clone());
    resources.insert("delimiter".into(), inner_params.delimiter.clone());
    resources.insert("marker".into(), inner_params.marker.clone());
    resources
}

pub fn get_is_end(inner_params: &ListQuery, len: usize) -> bool {
    let current_len: usize = match inner_params.max_keys.clone() {
        Some(max_keys) => max_keys.parse::<usize>().unwrap_or(100),
        None => 100,
    };
    len < current_len
}

pub fn get_last_marker<'a>(
    contents_ref: &'a Vec<object::Object>,
    common_prefix_ref: &'a Vec<object::CommonPrefix>,
) -> &'a str {
    if contents_ref.len() == 0 {
        if common_prefix_ref.len() != 0 {
            common_prefix_ref
                .get(common_prefix_ref.len() - 1)
                .unwrap()
                .prefix()
        } else {
            ""
        }
    } else {
        contents_ref.get(contents_ref.len() - 1).unwrap().key()
    }
}

pub fn get_img_url(key: &String) -> String {
    dotenv().ok();
    let bucket = env::var("BUCKET").expect("BUCKET is not defined");
    let region = env::var("REGION").expect("REGION is not defined");
    // 拼接img_url
    format!(
        "https://{}.{}/{}",
        bucket,
        region.replacen("https://", "", 1),
        key
    )
}

// 转换字段
pub fn transform_list_object_data_to_camel_case(
    list: ListObjects,
    last_marker: &str,
    is_end: bool,
) -> ListObjectCamelCase {
    let contents: Vec<Content> = list
        .contents()
        .iter()
        .map(|item| {
            Content::new(
                item.key().into(),
                item.last_modified().into(),
                item.size().into(),
                item.e_tag().into(),
                item.r#type().into(),
                item.storage_class().into(),
                Owner::new(item.id().into(), item.display_name().into()),
                list.prefix(),
            )
        })
        .filter(|item| {
            // 过滤当前文件夹本身
            item.name().len() != 0
        })
        .collect();

    let common_prefixed: Vec<CommonPrefix> = list
        .common_prefixes()
        .iter()
        .map(|item| CommonPrefix::new(item.prefix().to_string()))
        .collect();

    ListObjectCamelCase::new(
        list.name().to_string(),
        list.delimiter().into(),
        list.prefix().into(),
        list.marker().into(),
        list.max_keys().into(),
        list.is_truncated().into(),
        contents,
        common_prefixed,
        last_marker.into(),
        is_end,
    )
}
