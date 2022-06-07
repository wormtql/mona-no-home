use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::RwLock;
use chrono::{DateTime, Duration, Utc};

pub struct CreateRepoCount {
    pub min_interval: Duration,
    // pub max_time_per_interval: usize,
    // pub count: HashMap<SocketAddr, usize>,
    pub last_operation: RwLock<HashMap<SocketAddr, DateTime<Utc>>>
}

impl CreateRepoCount {
    pub fn new(max_interval: Duration) -> CreateRepoCount {
        CreateRepoCount {
            min_interval: max_interval,
            last_operation: RwLock::new(HashMap::new())
        }
    }

    pub fn try_ip(&self, addr: SocketAddr) -> bool {
        let contains_key = {
            let handle = self.last_operation.read().unwrap();
            handle.contains_key(&addr)
        };

        if !contains_key {
            let mut handle = self.last_operation.write().unwrap();
            handle.insert(addr, chrono::offset::Utc::now());
            true
        } else {
            let now = Utc::now();
            let delta = {
                let handle = self.last_operation.read().unwrap();
                let last = handle.get(&addr).unwrap();
                let delta = now - *last;
                delta
            };

            if delta < self.min_interval {
                false
            } else {
                let mut handle = self.last_operation.write().unwrap();
                handle.insert(addr, now);
                true
            }
        }
    }

    pub fn get_now_and_expire(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        let now = Utc::now();
        let expire = now + self.min_interval;
        (now, expire)
    }
}