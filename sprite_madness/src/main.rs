use bevy::prelude::*;

const SCENE_WIDTH: f32 = 1024.0;
const SCENE_HEIGHT: f32 = 768.0;

fn main() {
    App::new()
    .insert_resource(WindowDescriptor {
        width: SCENE_WIDTH,
        height: SCENE_HEIGHT,
        title: "Dungeon X".to_string(),
        present_mode: bevy::window::PresentMode::Fifo,
        resizable: false,
        ..Default::default()
    })
    .add_startup_system(setup)
    .run();
}

fn setup(mut commands: Commands){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
