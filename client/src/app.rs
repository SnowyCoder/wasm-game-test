use std::time::Duration;

use cgmath::Vector3;
use specs::{Dispatcher, DispatcherBuilder, World};
use specs::prelude::*;
use specs::shrev::EventChannel;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::graphics::GraphicContext;
use crate::graphics::renderer::{ActiveCamera, RenderBody, RenderSystem};
use crate::graphics::model::RenderModel;
use crate::input::{KeyboardEvent, MouseMoveEvent, ResizeEvent};
use crate::physics::player_move::PlayerMoveSystem;
use crate::physics::system::{BodyLocation, UpdateLocation, Velocity};
use crate::utils::RefClone;

#[derive(Debug, Default)]
pub struct DeltaTime(pub Duration);


pub struct App {
    pub world: World,
    pub dispatcher: Dispatcher<'static, 'static>,
    pub canvas: HtmlCanvasElement,
}

impl App {
    pub fn create() -> Result<App, JsValue> {
        let mut world = World::new();
        world.insert(DeltaTime(Duration::from_nanos(0)));
        world.insert(ActiveCamera(None));
        world.insert(EventChannel::<KeyboardEvent>::new());
        world.insert(EventChannel::<MouseMoveEvent>::new());
        world.insert(EventChannel::<ResizeEvent>::new());
        world.register::<BodyLocation>();
        world.register::<Velocity>();
        world.register::<RenderBody>();

        let graphics = GraphicContext::from_canvas("canvas")?;

        let render_system = RenderSystem::new(graphics, &mut world);

        let graphics = &render_system.gctx;

        let canvas = graphics.canvas.ref_clone();

        let player = world.create_entity()
            .with(BodyLocation::at_pos(Vector3 { x: 0.0, y: 0.0, z: 5.0 }))
            .build();

        let cube = world.create_entity()
            .with(BodyLocation::zero())
            .with(RenderBody::from_model(RenderModel::new(graphics, CUBE_VERTICES.as_ref())))
            .build();

        {
            let mut active_camera = world.write_resource::<ActiveCamera>();
            active_camera.0 = Some(player);
        }

        let dispatcher = DispatcherBuilder::new()
            .with(UpdateLocation, "update_location", &[])
            .with(PlayerMoveSystem::new(&mut world), "player_move", &[])
            .with_thread_local(render_system)
            .build();

        //dispatcher.setup(&mut world);

        Ok(App {
            world,
            dispatcher,
            canvas,
        })
    }

    pub fn update(&mut self, delta: Duration) {
        {// Update delta
            let deltatime = self.world.get_mut::<DeltaTime>().unwrap();
            deltatime.0 = delta
        }

        self.dispatcher.dispatch(&self.world);
        self.world.maintain();
    }
}


const CUBE_VERTICES: [f32; 12*3*3] = [
    -1.0, -1.0, -1.0,
    -1.0, -1.0,  1.0,
    -1.0,  1.0,  1.0,

    1.0,  1.0, -1.0,
    -1.0, -1.0, -1.0,
    -1.0,  1.0, -1.0,

    1.0, -1.0,  1.0,
    -1.0, -1.0, -1.0,
    1.0, -1.0, -1.0,

    1.0,  1.0, -1.0,
    1.0, -1.0, -1.0,
    -1.0, -1.0, -1.0,

    -1.0, -1.0, -1.0,
    -1.0,  1.0,  1.0,
    -1.0,  1.0, -1.0,

    1.0, -1.0,  1.0,
    -1.0, -1.0,  1.0,
    -1.0, -1.0, -1.0,

    -1.0,  1.0,  1.0,
    -1.0, -1.0,  1.0,
    1.0, -1.0,  1.0,

    1.0,  1.0,  1.0,
    1.0, -1.0, -1.0,
    1.0,  1.0, -1.0,

    1.0, -1.0, -1.0,
    1.0,  1.0,  1.0,
    1.0, -1.0,  1.0,

    1.0,  1.0,  1.0,
    1.0,  1.0, -1.0,
    -1.0,  1.0, -1.0,

    1.0,  1.0,  1.0,
    -1.0,  1.0, -1.0,
    -1.0,  1.0,  1.0,

    1.0,  1.0,  1.0,
    -1.0,  1.0,  1.0,
    1.0, -1.0,  1.0,
];
