use bevy::{prelude::*};
use bevy::{
    input::{keyboard::KeyCode, Input},
};

use wasm_bindgen::prelude::*;

use bevy::ecs::Stage;
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

fn js_log(s: &str) {
    log(s);
}

fn my_run_once(mut app: App) {
    info!("run_once starting");
    println!("run_once starting");
    js_log("run_once hello world");

    let mut bytes: Vec<u8> = vec!();
    let ptr = &app as *const App;
    let nb_bytes = std::mem::size_of::<App>();
    let starting_pointer = ptr as *const u8;
    for byte_index in 0..nb_bytes {
        unsafe {
            bytes.push(*starting_pointer.offset(byte_index as isize));
        }
    }

    println!("{:#04X?}", bytes);
    info!("{:#04X?}", bytes);

    app.update();

    info!("run_once starting");
    println!("run_once starting");
    js_log("run_once bye world");
}


#[wasm_bindgen(start)]
pub fn start() {

    info!("starting");
    println!("starting");
    js_log("hello world");
    let mut app_builder = bevy::app::App::build();

    app_builder
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
        

    //app_builder.run();

    let mut app = std::mem::take(&mut app_builder.app);
    

    // let first_runner = std::mem::replace(&mut app.runner, Box::new(my_run_once));
    // let second_runner = std::mem::replace(&mut app.runner, Box::new(my_run_once));
    // let third_runner = std::mem::replace(&mut app.runner, first_runner);
    // let fourth_runner = std::mem::replace(&mut app.runner, second_runner);

    // let runner = third_runner;
    // (runner)(app);

    app.run();


    info!("initialize_and_run");
    println!("initialize_and_run");
    js_log("initialize_and_run");


    //app.schedule.initialize_and_run(&mut app.world, &mut app.resources);
    // app.schedule.initialize(&mut app.world, &mut app.resources);
    // app.schedule.initialize(&mut app.world, &mut app.resources);
    // app.schedule.run(&mut app.world, &mut app.resources);

    //app.run();

    info!("ending");
    println!("ending");
    js_log("end world");
}

//see https://github.com/smokku/bevy/blob/master/examples/wasm/winit_wasm.rs
//and https://github.com/bevyengine/bevy/blob/master/examples/app/custom_loop.rs
//and https://github.com/bevyengine/bevy/blob/841755aaf23acfd55b375c37390daeb302c5b30b/examples/wasm/assets_wasm.rs
