// 应用程序状态
use oss_rust_sdk::prelude::*;
use std::sync::Mutex;

use super::client::OssClient;

pub struct AppState {
    // 健康检查, 不可变， 所有线程均持有
    pub health_check_response: String,
    // 可变的, 是一个数值, 这里使用 Mutex
    // ? Mutex 是标准库提供的一个保障线程通信的一个机制, 也就是在修改 visit_count 之前, 当前线程必须持有数据的控制权, 就是由 Mutex 完成的
    pub visit_count: Mutex<u32>,
    pub oss_client: OssClient,
}

pub const SUCCESS_CODE: u32 = 0;
