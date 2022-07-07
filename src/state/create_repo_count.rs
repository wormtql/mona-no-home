use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, SocketAddr};
use std::sync::{LockResult, RwLock};
use chrono::{DateTime, Duration, Utc};

pub struct CreateRepoCount {
    pub min_interval: Duration,
    // pub max_time_per_interval: usize,
    // pub count: HashMap<SocketAddr, usize>,
    pub last_operation: RwLock<HashMap<IpAddr, DateTime<Utc>>>
}

impl CreateRepoCount {
    pub fn new(max_interval: Duration) -> CreateRepoCount {
        CreateRepoCount {
            min_interval: max_interval,
            last_operation: RwLock::new(HashMap::new())
        }
    }

    pub fn cleanup_ip(&self) {
        match self.last_operation.write() {
            LockResult::Ok(mut handle) => {
                let now = Utc::now();

                let mut del_keys = HashSet::new();
                for (&k, v) in handle.iter() {
                    if now - *v > self.min_interval {
                        del_keys.insert(k);
                    }
                }

                for key in del_keys.into_iter() {
                    handle.remove(&key);
                }
            },
            LockResult::Err(e) => {
                return;
            }
        }
    }

    pub fn try_ip(&self, addr: IpAddr) -> bool {
        let contains_key = {
            let handle = self.last_operation.read().unwrap();
            handle.contains_key(&addr)
        };

        let result = if !contains_key {
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
        };

        self.cleanup_ip();
        result
    }

    pub fn get_now_and_expire(&self) -> (DateTime<Utc>, DateTime<Utc>) {
        let now = Utc::now();
        let expire = now + self.min_interval;
        (now, expire)
    }
}