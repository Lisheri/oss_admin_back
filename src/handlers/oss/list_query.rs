// use std::rc::Rc;

// use actix_web::web;
// use std::env;
// use dotenv::dotenv;
// 主要用于将原有库中的 PascalCase 类型字段转换为 camelCase
// use oss_rust_sdk::prelude::*;
use serde::{Deserialize, Serialize};

use super::utils::get_img_url;
use regex::Regex;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListQuery {
    pub prefix: Option<String>,    // 查询前缀
    pub max_keys: Option<String>,  // 分页条数
    pub delimiter: Option<String>, // 文件夹分隔符
    pub marker: Option<String>,    // 分页操作专用
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutFolderBody {
    pub folder_name: Option<String>, // 文件夹名称
    pub prefix: Option<String>,      // 查询前缀
    pub max_keys: Option<String>,    // 分页条数
    pub delimiter: Option<String>,   // 文件夹分隔符
    pub marker: Option<String>,      // 分页操作专用
}

impl From<&PutFolderBody> for ListQuery {
    fn from(folder_body: &PutFolderBody) -> Self {
        Self {
            prefix: folder_body.prefix.clone(),
            max_keys: folder_body.max_keys.clone(),
            delimiter: folder_body.delimiter.clone(),
            marker: folder_body.marker.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonPrefix {
    prefix: String,
}

impl CommonPrefix {
    pub fn new(prefix: String) -> Self {
        Self { prefix }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    #[serde(alias = "id")]
    pub id: String,
    pub display_name: String,
}

impl Owner {
    pub fn new(id: String, display_name: String) -> Self {
        Self { id, display_name }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    name: String,
    last_modified: String,
    size: usize,
    e_tag: String,
    r#type: String,
    storage_class: String,
    owner: Owner,
    img_url: String,
}

impl Content {
    pub fn new(
        key: String,
        last_modified: String,
        size: usize,
        e_tag: String,
        r#type: String,
        storage_class: String,
        owner: Owner,
        prefix: &str, // 用于过滤name的前缀
    ) -> Self {
        // 获取img_url
        let img_url = get_img_url(&key);
        // 过滤name前缀
        let filter_reg = Regex::new(prefix).unwrap();
        let name = filter_reg.replace(&key, "").to_string();
        Self {
            name,
            last_modified,
            size,
            e_tag,
            r#type,
            storage_class,
            owner,
            img_url,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListObjectCamelCase {
    name: String,
    delimiter: String,
    prefix: String,
    marker: String,
    max_keys: String,
    is_truncated: bool,
    last_marker: String, // 上一页最后一个, 用于分页查询
    is_end: bool,        // 是否查询完成
    contents: Vec<Content>,
    common_prefixes: Vec<CommonPrefix>,
}

impl ListObjectCamelCase {
    pub fn new(
        name: String,
        delimiter: String,
        prefix: String,
        marker: String,
        max_keys: String,
        is_truncated: bool,
        contents: Vec<Content>,
        common_prefixes: Vec<CommonPrefix>,
        last_marker: String,
        is_end: bool,
    ) -> Self {
        Self {
            name,
            delimiter,
            prefix,
            marker,
            max_keys,
            is_truncated,
            contents,
            common_prefixes,
            last_marker,
            is_end,
        }
    }
}
