use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Defacto".into(),
                resolution: WindowResolution::new(800, 600),
                present_mode: PresentMode::Fifo,
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .run();
}
