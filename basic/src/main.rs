use std::f64::consts::PI;

use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
use bevy_prototype_lyon::prelude::*;

#[derive(Component)]
struct MyComponent;

#[derive(Inspectable)]
struct Data {
    #[inspectable(min = 5, max = 100)]
    sides: usize,
    #[inspectable(min = 5.0, max = 100.0)]
    size: f32,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            sides: 6,
            size: 50.0,
        }
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 640.0,
            height: 480.0,
            title: "Teste".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(InspectorPlugin::<Data>::new())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_primitives)
        .add_system(handle_mouse_clicks)
        .add_system(change_size_system)
        .run();
}

fn setup_primitives(data: Res<Data>, mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: data.sides,
        feature: shapes::RegularPolygonFeature::Radius(data.size),
        ..shapes::RegularPolygon::default()
    };
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 10.0),
            },
            Transform::default(),
        ))
        .insert(MyComponent);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn handle_mouse_clicks(mouse_input: Res<Input<MouseButton>>, windows: Res<Windows>) {
    let win = windows.get_primary().expect("no primary window");
    if mouse_input.just_pressed(MouseButton::Left) {
        println!("click at {:?}", win.cursor_position());
    }
}

fn change_size_system(data: Res<Data>, mut query: Query<&mut Path>, time: Res<Time>) {
    //let sides = ((time.seconds_since_startup() - PI * 2.5).sin() * 2.5 + 5.5).round() as usize;
    let sides = data.sides;
    let size = data.size;

    for mut path in query.iter_mut() {
        let polygon = shapes::RegularPolygon {
            sides,
            feature: shapes::RegularPolygonFeature::Radius(size),
            ..shapes::RegularPolygon::default()
        };

        *path = ShapePath::build_as(&polygon);
    }
}
