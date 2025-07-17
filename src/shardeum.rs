use crate::config;
use std::sync::Arc;

pub struct Shardeum {
    pub config: Arc<config::Config>,
}
