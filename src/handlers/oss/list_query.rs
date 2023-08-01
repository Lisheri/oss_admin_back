use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub prefix: String,   // 查询前缀
    pub max_keys: String, // 分页条数
}
