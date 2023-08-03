use crate::error::MyError;
use dotenv::dotenv;
// use oss_rust_sdk::{async_object::AsyncObjectAPI, oss::OSS, prelude::ListObjects};
use oss_rust_sdk::prelude::*;
use std::{
    borrow::Cow,
    collections::HashMap,
    env,
    sync::{mpsc, Arc},
    thread,
};

#[derive(Clone)]
pub struct OssClient {
    // 整个运行周期 instance都不应该被回收
    instance: Arc<OSS<'static>>,
}

impl OssClient {
    pub fn new() -> Self {
        dotenv().ok();
        let access_key_id = env::var("ACCESS_KEY_ID").expect("ACCESS_KEY_ID is not defined");
        let access_key_secret =
            env::var("ACCESS_KEY_SECRET").expect("ACCESS_KEY_ID is not defined");
        let bucket = env::var("BUCKET").expect("BUCKET is not defined");
        let region = env::var("REGION").expect("REGION is not defined");
        Self {
            instance: Arc::new(OSS::new(
                Into::<Cow<str>>::into(access_key_id),
                Into::<Cow<str>>::into(access_key_secret),
                Into::<Cow<str>>::into(region),
                Into::<Cow<str>>::into(bucket),
            )),
        }
    }

    // 查询列表数据
    pub fn get_list(
        self,
        headers: HashMap<String, String>,
        resources: HashMap<String, Option<String>>,
    ) -> Result<ListObjects, MyError> {
        // 线程间传递消息
        let (tx, rx) = mpsc::channel();

        let rt = thread::spawn(move || {
            let buckets = self.instance.list_object(headers, resources);
            tx.send(buckets.unwrap()).unwrap();
        });

        rt.join().unwrap();

        match rx.recv() {
            Ok(list) => Ok(list),
            Err(err) => {
                // 错误传递
                println!("err msg is: {:?}", err);
                Err(MyError::ActixError(err.to_string()))
            }
        }

        // async异步方法(2选1)
        // match self.instance.list_object(headers, resources).await {
        //     Ok(list) => Ok(list),
        //     Err(err) => {
        //         println!("err msg is: {:?}", err);
        //         Err(MyError::ActixError(err.to_string()))
        //     }
        // }
    }
}
