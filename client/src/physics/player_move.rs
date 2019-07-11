use specs::prelude::*;
use specs::{System, ReaderId, Read};
use specs::shrev::EventChannel;
use crate::input::{KeyboardEvent, KeyState, MouseMoveEvent};
use crate::physics::system::BodyLocation;
use crate::graphics::renderer::ActiveCamera;
use bitflags::bitflags;
use cgmath::prelude::*;
use cgmath::{Vector3, Deg};
use crate::console_log;


bitflags! {
    #[derive(Default)]
    struct MoveDirection: u8 {
        const FORWARD = 0b0001;
        const BACK  = 0b0010;
        const LEFT  = 0b0100;
        const RIGHT = 0b1000;
    }
}

pub struct PlayerMoveSystem {
    keyboard_reader: ReaderId<KeyboardEvent>,
    mouse_reader: ReaderId<MouseMoveEvent>,
    dir: MoveDirection,
}


impl PlayerMoveSystem {
    pub fn new(world: &mut World) -> PlayerMoveSystem {
        PlayerMoveSystem {
            keyboard_reader: world.write_resource::<EventChannel<KeyboardEvent>>().register_reader(),
            mouse_reader: world.write_resource::<EventChannel<MouseMoveEvent>>().register_reader(),
            dir: MoveDirection::empty(),
        }
    }

    fn parse_key(key: &str) -> MoveDirection{
        match key {
            "W" => MoveDirection::FORWARD,
            "S" => MoveDirection::BACK,
            "A" => MoveDirection::LEFT,
            "D" => MoveDirection::RIGHT,
            _ => MoveDirection::empty()
        }
    }

    fn apply_vel(&self, player: &mut BodyLocation) {
        const VEL: f32 = 0.1;

        let mut dir: Vector3<f32> = Vector3::zero() as Vector3<f32>;

        if self.dir.contains(MoveDirection::FORWARD) {
            dir.z -= 1.0;
        }
        if self.dir.contains(MoveDirection::BACK) {
            dir.z += 1.0;
        }
        if self.dir.contains(MoveDirection::LEFT) {
            dir.x -= 1.0;
        }
        if self.dir.contains(MoveDirection::RIGHT) {
            dir.x += 1.0;
        }


        if !dir.is_zero() {
            console_log!("Vel: {:?}", dir);
            player.forward(dir.normalize() * VEL);
        }
    }
}

impl<'a> System<'a> for PlayerMoveSystem {
    type SystemData = (
        WriteStorage<'a, BodyLocation>,
        Read<'a, ActiveCamera>,
        Read<'a, EventChannel<KeyboardEvent>>,
        Read<'a, EventChannel<MouseMoveEvent>>,
    );

    fn run(&mut self, (mut location, camera, keyboard_events, mouse_events): Self::SystemData) {
        let camera_loc = camera.0.and_then(|e| location.get_mut(e));

        // Update direction
        for event in keyboard_events.read(&mut self.keyboard_reader) {
            let dir = PlayerMoveSystem::parse_key(&event.key.to_uppercase());
            if event.state == KeyState::DOWN {
                self.dir.insert(dir);
            } else {
                self.dir.remove(dir);
            }
        }
        //self.graphics.camera.rotate(Deg(dx as f32 * PREC), Deg(dy as f32 * PREC));
        // Update rotation


        if let Some(loc) = camera_loc {
            let sensibility = 0.05; // TODO: resource
            for event in mouse_events.read(&mut self.mouse_reader) {
                loc.rotate(Deg(event.dx as f32 * sensibility), Deg(event.dy as f32 * sensibility));
            }


            self.apply_vel(loc);
        } else {
            console_log!("No active player found");
            mouse_events.read(&mut self.mouse_reader);
        }
    }
}
