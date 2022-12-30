use bevy::prelude::NonSend;
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use winit::window::Icon;

pub fn set_window_icon(
    windows: NonSend<WinitWindows>,
) {
    let primary = windows.get_window(WindowId::primary()).unwrap();

    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icon.ico")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary.set_window_icon(Some(icon));
    primary.set_title("Squad of Five");
}