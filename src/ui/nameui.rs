use std::sync::Arc;

use druid::{
    widget::{Flex, Label},
    Widget, WidgetExt,
};

use crate::data::appstate::Name;

pub fn build_name() -> impl Widget<Name> {
    Flex::row()
        .with_child(
            Label::new(|data: &Arc<str>, _env: &_| format!("Hello {}", data)).lens(Name::fname),
        )
        .with_default_spacer()
        .with_child(Label::new(|data: &Arc<str>, _env: &_| format!("{}", data)).lens(Name::lname))
}
