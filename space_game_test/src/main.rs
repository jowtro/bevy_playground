use bevy::prelude::*;

const SCENE_WIDTH: f32 = 1024.;
const SCENE_HEIGHT: f32 = 768.;

#[derive(Component)]
struct Player {
    x: i32,
    y: i32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            x: (SCENE_WIDTH / 2.0) as i32,
            y: (SCENE_HEIGHT / 2.0) as i32,
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            width: SCENE_WIDTH,
            height: SCENE_HEIGHT,
            title: "Space Game lab".to_string(),
            present_mode: bevy::window::PresentMode::Fifo,
            resizable: false,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add 2D camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("space_ship1.png"),
            ..default()
        })
        .insert(Player { ..default() });
}