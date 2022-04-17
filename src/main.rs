use std::sync::Arc;

use druid::{
    self,
    im::Vector,
    widget::{Button, Flex, Label, List},
    Data, Lens, PlatformError, Widget, WidgetExt, WindowDesc,
};

#[derive(Data, Clone, Lens)]
struct AppState {
    name: Name,
    ages: Vector<u32>,
}

#[derive(Clone, Data, Lens)]
struct Name {
    fname: Arc<str>,
    lname: Arc<str>,
}

fn build_name() -> impl Widget<Name> {
    Flex::row()
        .with_child(
            Label::new(|data: &Arc<str>, _env: &_| format!("Hello {}", data)).lens(Name::fname),
        )
        .with_child(Label::new(|data: &Arc<str>, _env: &_| format!("{}", data)).lens(Name::lname))
}

fn build_list() -> impl Widget<Vector<u32>> {
    List::new(|| Label::new(|a: &u32, _env: &_| format!("{}", a)))
}

fn build_ui() -> WindowDesc<AppState> {
    WindowDesc::new(
        Flex::column()
            .with_child(build_name().lens(AppState::name))
            .with_child(build_list().lens(AppState::ages))
            .with_child(Button::new("Add").on_click(
                |_ctx: &mut _, appstate: &mut AppState, _env: &_| appstate.ages.extend([1, 2, 3]),
            )),
    )
}

fn main() -> Result<(), PlatformError> {
    druid::AppLauncher::with_window(build_ui()).launch(AppState {
        name: Name {
            fname: "Jack".into(),
            lname: "Dorsey".into(),
        },
        ages: Vector::new(),
    })
}
