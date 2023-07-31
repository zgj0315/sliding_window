use std::{
    num::NonZeroUsize,
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};

use dev_util::log::log_init_with_level;
use lru::LruCache;

fn main() {
    log_init_with_level(dev_util::log::Level::INFO);
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        log_detect(rx);
    });
    log::info!("Hello, world!");

    for _ in 0..20 {
        let api_log = ApiLog {
            src_ip: String::from("172.16.0.29"),
            session_id: String::from("session_id_123456"),
            api_addr: String::from("api/get_user_info/123456"),
            ts: chrono::Local::now().timestamp_millis(),
        };
        thread::sleep(Duration::from_secs(1));
        let _ = tx.send(api_log);
    }
    for _ in 0..200 {
        let api_log = ApiLog {
            src_ip: String::from("172.16.0.29"),
            session_id: String::from("session_id_123456"),
            api_addr: String::from("api/get_user_info/123456"),
            ts: chrono::Local::now().timestamp_millis(),
        };
        thread::sleep(Duration::from_millis(10));
        let _ = tx.send(api_log);
    }
    for _ in 0..20 {
        let api_log = ApiLog {
            src_ip: String::from("172.16.0.29"),
            session_id: String::from("session_id_123456"),
            api_addr: String::from("api/get_user_info/123456"),
            ts: chrono::Local::now().timestamp_millis(),
        };
        thread::sleep(Duration::from_secs(1));
        let _ = tx.send(api_log);
    }
}

#[derive(Debug, Clone)]
struct ApiLog {
    src_ip: String,
    session_id: String,
    api_addr: String,
    ts: i64,
}

#[derive(Debug, Clone, Copy)]
struct Delta {
    count_now: u64,
    count_last: u64,
    ts_last: i64,
}

fn log_detect(rx: Receiver<ApiLog>) {
    let window = 10_000;
    let threshold = 20;

    let mut cache: LruCache<String, Delta> = LruCache::new(NonZeroUsize::new(1024).unwrap());

    while let Ok(api_log) = rx.recv() {
        let key = api_log.src_ip;
        match cache.get_mut(&key) {
            Some(delta) => {
                delta.count_now += 1;

                // 突破了阈值
                if delta.count_now > delta.count_last + threshold {
                    log::info!("threshold: {:?}", delta);
                    delta.count_last = delta.count_now;
                    delta.count_now = 0;
                    delta.ts_last = api_log.ts;
                    continue;
                }
                // 突破了窗口
                if api_log.ts > delta.ts_last + window {
                    log::info!("window: {:?}", delta);
                    delta.count_last = delta.count_now;
                    delta.count_now = 0;
                    delta.ts_last = api_log.ts;
                    continue;
                }
            }
            None => {
                let delta = Delta {
                    count_now: 1,
                    count_last: 0,
                    ts_last: api_log.ts,
                };
                cache.push(key, delta);
            }
        }
    }
}
