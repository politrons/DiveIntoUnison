use bevy::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy WebAssembly Example".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.5, 0.5, 1.0),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}
