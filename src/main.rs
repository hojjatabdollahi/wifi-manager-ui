use core::time;
use std::sync::Arc;
extern crate wifiscanner;

use std::thread;

use druid::{
    self,
    im::Vector,
    widget::{Button, Either, Flex, Label, List, Scroll, Spinner},
    AppDelegate, Data, ExtEventSink, Handled, Lens, PlatformError, Selector, Target, Widget,
    WidgetExt, WindowDesc,
};
use std::process::Command;

const WIFI_LIST_READY: Selector<Vector<WifiItem>> = Selector::new("wifi_list_ready");

#[derive(Data, Clone, Lens)]
struct AppState {
    name: Name,
    wifis: Vector<WifiItem>,
    wifi_processing: bool,
}

#[derive(Clone, Data, Lens)]
struct Name {
    fname: Arc<str>,
    lname: Arc<str>,
}

#[derive(Clone, Data, Lens)]
struct WifiItem {
    inuse: bool,
    security: Arc<str>,
    ssid: Arc<str>,
}

fn build_name() -> impl Widget<Name> {
    Flex::row()
        .with_child(
            Label::new(|data: &Arc<str>, _env: &_| format!("Hello {}", data)).lens(Name::fname),
        )
        .with_default_spacer()
        .with_child(Label::new(|data: &Arc<str>, _env: &_| format!("{}", data)).lens(Name::lname))
}

fn build_list() -> impl Widget<Vector<WifiItem>> {
    List::new(|| {
        Flex::row()
            .with_child(
                Label::new(|a: &bool, _env: &_| {
                    format!("{}", if *a { "Connected" } else { "Not Connected" })
                })
                .lens(WifiItem::inuse),
            )
            .with_default_spacer()
            .with_child(
                Label::new(|a: &Arc<str>, _env: &_| format!("{}", a.clone())).lens(WifiItem::ssid),
            )
    })
}

fn build_ui() -> WindowDesc<AppState> {
    WindowDesc::new(
        Flex::column()
            .with_child(build_name().lens(AppState::name))
            .with_child(
                Button::new("Scan")
                    .on_click(|ctx: &mut _, appstate: &mut AppState, _env: &_| {
                        appstate.wifi_processing = true;
                        wrapped_wifi_list(ctx.get_external_handle())
                    })
                    .disabled_if(|appstate, _env| appstate.wifi_processing),
            )
            .with_child(Either::new(
                |appstate, _env| appstate.wifi_processing,
                Flex::column()
                    .with_child(Label::new("getting the wifi list").padding(5.0))
                    .with_child(Spinner::new()),
                Scroll::new(build_list().lens(AppState::wifis)),
            )),
    )
}

fn wrapped_wifi_list(sink: ExtEventSink) {
    thread::spawn(move || {
        let output = Command::new("nmcli")
            .arg("--terse")
            .arg("--fields")
            .arg("IN-USE,SSID,SECURITY")
            .arg("dev")
            .arg("wifi")
            .output()
            .expect("Couldn't get the wifi list");
        let data = String::from_utf8_lossy(&output.stdout);
        let a = data
            .split('\n')
            .collect::<Vec<_>>()
            .into_iter()
            .map(|a| a.split(':').collect::<Vec<_>>())
            .collect::<Vec<Vec<&str>>>();
        let mut networks = Vector::<WifiItem>::new();
        for net in a {
            if net.len() != 3 {
                eprintln!("Not3? {:?}", net);
                continue;
            }
            let wifi = WifiItem {
                inuse: net[0].contains('*'),
                ssid: net[1].into(),
                security: net[2].into(),
            };
            networks.push_back(wifi);
        }
        sink.submit_command(WIFI_LIST_READY, networks, Target::Auto)
    });
}

struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _target: Target,
        cmd: &druid::Command,
        data: &mut AppState,
        _env: &druid::Env,
    ) -> Handled {
        if let Some(networks) = cmd.get(WIFI_LIST_READY) {
            data.wifis = networks.clone();
            data.wifi_processing = false;
            Handled::Yes
        } else {
            Handled::No
        }
    }
}

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
