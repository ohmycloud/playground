use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use chrono::{DateTime, Utc};

// This will be the request limit (per limit) for a user to access an endpoint
// If the user attempts to go beyond this limit, we should return an error
const REQUEST_LIMIT: usize = 120;

#[derive(Clone, Default)]
pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<IpAddr, Vec<DateTime<Utc>>>>>,
}

impl RateLimiter {
    fn check_if_rate_limited(&self, ip_addr: IpAddr) -> Result<(), String> {
        // We only want to keep timestamps from up to 60 secnonds ago
        let throttle_time_limit = Utc::now() - std::time::Duration::from_secs(60);
        // Lock our hashmap by using `.lock()` which gives us write access
        let mut requests_hashmap = self.requests.lock().unwrap();
        let mut requests_for_ip = requests_hashmap
            // grab the entry here and allow us to modify it in place
            .entry(ip_addr)
            // if the entry is empty, insert a vec with the current timestamp
            .or_insert(Vec::new());

        requests_for_ip.retain(|x| x.to_utc() > throttle_time_limit);
        requests_for_ip.push(Utc::now());

        if requests_for_ip.len() > REQUEST_LIMIT {
            return Err("IP is rate limited :(".to_string());
        }
        Ok(())
    }
}

fn main() {
    let rate_limiter = RateLimiter::default();
    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    // here we request 120 times - our request limit
    for _ in 1..80 {
        assert!(rate_limiter.check_if_rate_limited(localhost_v4).is_ok());
    }

    // wait 30 seconds
    std::thread::sleep(std::time::Duration::from_secs(30));

    // make another 40 requests here to satisfy request quota
    for _ in 1..40 {
        assert!(rate_limiter.check_if_rate_limited(localhost_v4).is_ok());
    }

    // wait another 30 seconds
    std::thread::sleep(std::time::Duration::from_secs(30));

    // now we can make another 80 requests
    for _ in 1..80 {
        assert!(rate_limiter.check_if_rate_limited(localhost_v4).is_ok())
    }

  // chrono = { version = "0.4.34", features = ["serde", "clock"] }
  // serde = { version = "1.0.196", features = ["derive"] }
}
