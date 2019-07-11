use specs::{Component, Entity, Join, Read, ReadStorage, System, VecStorage, ReaderId, World, WorldExt};
use specs::shrev::EventChannel;
use web_sys::WebGl2RenderingContext;

use crate::graphics::GraphicContext;
use crate::graphics::model::RenderModel;
use crate::physics::system::BodyLocation;
use crate::input::ResizeEvent;

pub struct RenderBody {
    // The only one that will use the model is the render system and it needs to run on a single
    // thread, therefore there will be only one thread at a time that accesses the model
    model: RenderModel,
}

unsafe impl Send for RenderBody {}
unsafe impl Sync for RenderBody {}

impl RenderBody {
    pub fn from_model(model: RenderModel) -> RenderBody {
        RenderBody {
            model
        }
    }
}

impl Component for RenderBody {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Default)]
pub struct ActiveCamera(pub Option<Entity>);


pub struct RenderSystem {
    pub gctx: GraphicContext,
    resize_reader: ReaderId<ResizeEvent>,
}

unsafe impl Send for RenderSystem {}

impl RenderSystem {
    pub fn new(graphics: GraphicContext, world: &mut World) -> RenderSystem {
        RenderSystem {
            gctx: graphics,
            resize_reader: world.write_resource::<EventChannel<ResizeEvent>>().register_reader(),
        }
    }
}

impl<'a > System<'a> for RenderSystem {
    type SystemData = (
        ReadStorage<'a, RenderBody>,
        ReadStorage<'a, BodyLocation>,
        Read<'a, ActiveCamera>,
        Read<'a, EventChannel<ResizeEvent>>,
    );

    fn run(&mut self, (body, location, camera, resize_events): Self::SystemData) {
        for event in resize_events.read(&mut self.resize_reader) {
            self.gctx.on_resize(event.width, event.height);
        }

        let graphics = &self.gctx;
        let gl = &graphics.gl;
        //self.gctx.gl.enable_vertex_attrib_array(0);

        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

        let camera_loc = camera.0
            .and_then(|e| location.get(e))
            .map(|e| (*e).clone())
            .unwrap_or(BodyLocation::zero());

        let world_to_screen = graphics.camera.to_matrix(&camera_loc);
        //console_log!("world_to_screen: {:?}", world_to_screen);

        gl.use_program(Some(&self.gctx.program));
        gl.uniform_matrix4fv_with_f32_array(
            Some(&graphics.world_to_screen_loc),
            false,
            world_to_screen.as_ref() as &[f32; 16]
        );


        for (body, location) in (&body, &location).join() {
            let body: &RenderBody = body;
            let model = &body.model;
            let model_to_world = location.model_to_world_matrix();

            gl.uniform_matrix4fv_with_f32_array(
                Some(&graphics.model_to_world_loc),
                false,
                model_to_world.as_ref() as &[f32; 16]
            );

            gl.bind_vertex_array(Some(&model.vao));
            gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, model.triangle_count as i32);
            gl.bind_vertex_array(None);
        }
    }
}
