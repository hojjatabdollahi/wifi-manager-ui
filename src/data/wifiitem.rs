use std::sync::Arc;

use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct WifiItem {
    pub inuse: bool,
    pub security: Arc<str>,
    pub ssid: Arc<str>,
    pub signal: u8,
}
