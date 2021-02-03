use crate::app_builder::AppBuilder;
use bevy_ecs::{prelude::*, Resources, Schedule, World};
#[cfg(feature = "trace")]
use bevy_utils::tracing::info_span;
use bevy_utils::tracing::*;

#[allow(clippy::needless_doctest_main)]
/// Containers of app logic and data
///
/// App store the ECS World, Resources, Schedule, and Executor. They also store the "run" function of the App, which
/// by default executes the App schedule once. Apps are constructed using the builder pattern.
///
/// ## Example
/// Here is a simple "Hello World" Bevy app:
/// ```
///use bevy_app::prelude::*;
///use bevy_ecs::prelude::*;
///
///fn main() {
///    App::build()
///        .add_system(hello_world_system.system())
///        .run();
///}
///
///fn hello_world_system() {
///    println!("hello world");
///}
/// ```
pub struct App {
    pub world: World,
    pub resources: Resources,
    pub runner: Box<dyn Fn(App)>,
    pub schedule: Schedule,
}

impl Default for App {
    fn default() -> Self {
        Self {
            world: Default::default(),
            resources: Default::default(),
            schedule: Default::default(),
            runner: Box::new(run_once),
        }
    }
}

fn run_once(mut app: App) {

    println!("run_once");
    warn!("run_once");
    println!("run_once");
    warn!("run_once");
    println!("run_once");
    warn!("run_once");
    println!("run_once");
    warn!("run_once");
    println!("run_once");
    warn!("run_once");
    println!("run_once");
    warn!("run_once");
    println!("run_once");
    warn!("run_once");


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
    warn!("{:#04X?}", bytes);


    app.update();
}

impl App {
    pub fn build() -> AppBuilder {
        AppBuilder::default()
    }

    pub fn update(&mut self) {


        println!("update");
        warn!("update");

        self.schedule
            .initialize_and_run(&mut self.world, &mut self.resources);
    }

    pub fn run(mut self) {


        println!("run");
        warn!("run");

        #[cfg(feature = "trace")]
        let bevy_app_run_span = info_span!("bevy_app");
        #[cfg(feature = "trace")]
        let _bevy_app_run_guard = bevy_app_run_span.enter();

        let runner = std::mem::replace(&mut self.runner, Box::new(run_once));
        (runner)(self);
    }
}

/// An event that indicates the app should exit. This will fully exit the app process.
#[derive(Debug, Clone)]
pub struct AppExit;
