mod utils;

use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet() {}

//  TODO: send canvas size to rust
//  TODO: sometimes hit `unreachable error` after window is open for a while, i
//  think we need to have actual singleton on react side
use bevy::prelude::*;
#[wasm_bindgen]
pub fn run(width: f32, height: f32) {
    App::new()
        .insert_resource(WindowDescriptor {
            canvas: Some("#bevy-canvas".into()),
            width,
            height,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(tick)
        .run();
}

#[derive(Component)]
struct CubeTag;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CubeTag);
}

fn tick(mut q: Query<(&mut Transform, &CubeTag)>, windows: Res<Windows>) {
    for mut cube in q.iter_mut() {
        let pos = &mut cube.0.translation;
        let window = windows.get_primary().unwrap();
        let cursor = window.cursor_position().unwrap_or_default();
        pos.x = cursor.x - window.width() / 2.;
        pos.y = cursor.y - window.height() / 2.;
    }
}
