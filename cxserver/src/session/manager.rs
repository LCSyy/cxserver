use std::collections::HashMap;
use std::sync::Mutex;

struct Manager {
    cookie_name: String,
    max_life_time: i64,
    provider: Provider,
}
