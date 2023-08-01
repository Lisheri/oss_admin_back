use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use std::thread;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub prefix: String,   // 查询前缀
    pub max_keys: String, // 分页条数
}

#[path = "../error/error.rs"]
mod error;

#[path = "../modules/ossClient/client.rs"]
mod client;

use client::OssClient;

#[derive(Debug, Serialize)]
struct ResponseResult<'a, T> {
    code: u32,
    msg: &'a str,
    data: T,
}

#[actix_rt::main]
async fn main() {
    let t1 = thread::spawn(move|| {
        get_all_list();
    });
    t1.join().unwrap();

    pub fn get_all_list() {
        let headers: HashMap<String, String> = HashMap::new();
        let mut resources: HashMap<String, Option<String>> = HashMap::new();
        resources.insert("prefix".into(), Some("images".to_string()));
        resources.insert("max_keys".into(), Some("10".into()));

        let _ = OssClient::new().get_list(headers, resources).map(|list| {
            println!("list is: {:?}", list);
        });
    }
}
