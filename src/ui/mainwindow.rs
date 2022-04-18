use std::{process::Command, thread};

use druid::{
    im::Vector,
    widget::{Button, Either, Flex, Label, Scroll, Spinner},
    AppDelegate, Color, ExtEventSink, Handled, Selector, Target, WidgetExt, WindowDesc,
};

use crate::data::{appstate::AppState, wifiitem::WifiItem};

use super::{icon::WIFI, nameui::build_name, wifilistui::build_list};
const WIFI_LIST_READY: Selector<Vector<WifiItem>> = Selector::new("wifi_list_ready");

pub fn build_ui() -> WindowDesc<AppState> {
    WindowDesc::new(
        Flex::row()
            .with_flex_spacer(0.25)
            .with_flex_child(
                Flex::column()
                    .with_child(build_name().lens(AppState::name))
                    .with_child(WIFI.scale((32.0, 32.0)).with_color(Color::rgb8(0, 128, 0)))
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
                        Scroll::new(build_list().lens(AppState::wifis)).fix_height(400.0),
                    )),
                0.50,
            )
            .with_flex_spacer(0.25),
    )
    .with_min_size((400.0, 600.0))
    .window_size((800.0, 600.0))
}

fn wrapped_wifi_list(sink: ExtEventSink) {
    thread::spawn(move || {
        let output = Command::new("nmcli")
            .arg("--terse")
            .arg("--fields")
            .arg("IN-USE,SSID,SECURITY,SIGNAL")
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
            if net.len() != 4 {
                eprintln!("Not3? {:?}", net);
                continue;
            }
            let wifi = WifiItem {
                inuse: net[0].contains('*'),
                ssid: net[1].into(),
                security: net[2].into(),
                signal: net[3].parse::<u8>().unwrap_or(0),
            };
            networks.push_back(wifi);
        }
        sink.submit_command(WIFI_LIST_READY, networks, Target::Auto)
    });
}

pub struct Delegate;

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
