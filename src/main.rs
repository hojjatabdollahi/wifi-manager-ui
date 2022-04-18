mod data;
mod ui;

extern crate wifiscanner;

use data::appstate::{AppState, Name};
use druid::{self, im::Vector, PlatformError};
use ui::mainwindow::{build_ui, Delegate};

fn main() -> Result<(), PlatformError> {
    druid::AppLauncher::with_window(build_ui().title("Wifi Scanner"))
        .delegate(Delegate {})
        .log_to_console()
        .launch(AppState {
            name: Name {
                fname: "Wifi".into(),
                lname: "Scanner".into(),
            },
            wifis: Vector::new(),
            wifi_processing: false,
        })
}
