use std::sync::Arc;

use super::wifiitem::WifiItem;
use druid::{im::Vector, Data, Lens};

#[derive(Data, Clone, Lens)]
pub struct AppState {
    pub name: Name,
    pub wifis: Vector<WifiItem>,
    pub wifi_processing: bool,
}

#[derive(Clone, Data, Lens)]
pub struct Name {
    pub fname: Arc<str>,
    pub lname: Arc<str>,
}
