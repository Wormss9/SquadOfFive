mod states;
use states::*;

mod icon;
use icon::set_window_icon;

use bevy::{
    prelude::*,
    winit::WinitSettings,
};

mod play;

use play::{Play, deal_cards};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(set_window_icon)
        .add_state(AppState::MainMenu)
        .add_state(TurnState::random())
        .run();
}