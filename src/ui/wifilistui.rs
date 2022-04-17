use std::sync::Arc;

use druid::{
    im::Vector,
    widget::{Flex, Label, List},
    Widget, WidgetExt,
};

use crate::data::wifiitem::WifiItem;

pub fn build_list() -> impl Widget<Vector<WifiItem>> {
    List::new(|| {
        Flex::row()
            .with_child(
                Label::new(|a: &bool, _env: &_| {
                    (if *a { "Connected" } else { "Not Connected" }).to_string()
                })
                .lens(WifiItem::inuse),
            )
            .with_default_spacer()
            .with_child(
                Label::new(|a: &Arc<str>, _env: &_| format!("{}", a.clone())).lens(WifiItem::ssid),
            )
    })
}
