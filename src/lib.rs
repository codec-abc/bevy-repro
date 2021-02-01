use std::{borrow::BorrowMut, ops::DerefMut, sync::{Arc, Mutex}};

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use bevy::{prelude::*};
use bevy::{
    input::{keyboard::KeyCode, Input},
};

use wasm_bindgen::prelude::*;

use bevy::render::camera::Camera;

/// This system prints 'A' key state
fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>, 
    time: Res<Time>, 
    mut cam_query: Query<(&Camera, &mut Transform)>
) 
{
    //info!("keyboard_input_system running!");
    if keyboard_input.pressed(KeyCode::A) {
        info!("'A' currently pressed");

        let delta_seconds = time.delta_seconds();

        for (_camera, mut transform) in cam_query.iter_mut() {
            let right = -transform.rotation * Vec3::unit_x();
            transform.translation += right * delta_seconds;
        }
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        info!("'A' just pressed");
    }

    if keyboard_input.just_released(KeyCode::A) {
        info!("'A' just released");
    }
}

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    commands
        // plane
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        // cube
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
            ..Default::default()
        })
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(-2.0, 2.5, 5.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen(start)]
pub fn start() {

    info!("starting");
    println!("starting");
    unsafe { log("hello world") };
    let mut app = bevy::app::App::build();

    app
        .add_resource(WindowDescriptor {
            width: 300.,
            height: 300.,
            #[cfg(target_arch = "wasm32")]
            canvas: Some("#bevy_canvas".into()),
            ..Default::default()
        })
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_webgl2::WebGL2Plugin)
        .add_system(keyboard_input_system.system())
        .add_startup_system(setup.system());

    app.run();
}

//see https://github.com/smokku/bevy/blob/master/examples/wasm/winit_wasm.rs
//and https://github.com/bevyengine/bevy/blob/master/examples/app/custom_loop.rs
//and https://github.com/bevyengine/bevy/blob/841755aaf23acfd55b375c37390daeb302c5b30b/examples/wasm/assets_wasm.rs
