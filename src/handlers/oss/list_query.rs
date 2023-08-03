// 主要用于将原有库中的 PascalCase 类型字段转换为 camelCase
// use oss_rust_sdk::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub prefix: Option<String>,    // 查询前缀
    pub max_keys: Option<String>,  // 分页条数
    pub delimiter: Option<String>, // 文件夹分隔符
    pub marker: Option<String>,    // 分页操作专用
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
pub struct Object {
    key: String,
    last_modified: String,
    size: usize,
    e_tag: String,
    r#type: String,
    storage_class: String,
    owner: Owner,
}

impl Object {
    pub fn new(
        key: String,
        last_modified: String,
        size: usize,
        e_tag: String,
        r#type: String,
        storage_class: String,
        owner: Owner,
    ) -> Self {
        Self {
            key,
            last_modified,
            size,
            e_tag,
            r#type,
            storage_class,
            owner,
        }
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
    is_end: bool, // 是否查询完成
    contents: Vec<Object>,
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
        contents: Vec<Object>,
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
            is_end
        }
    }
}
