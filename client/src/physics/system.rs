use specs::{Component, VecStorage, System, ReadStorage, WriteStorage, Join, Read};
use cgmath::prelude::*;
use cgmath::{Vector3, Matrix4, Deg, Rad};
use crate::app::DeltaTime;

#[derive(Debug, Clone)]
pub struct BodyLocation {
    pub pos: Vector3<f32>,
    pub yaw: Deg<f32>,
    pub pitch: Deg<f32>,
}

impl BodyLocation {
    pub fn zero() -> BodyLocation {
        BodyLocation {
            pos: Vector3::zero(),
            yaw: Deg::zero(),
            pitch: Deg::zero(),
        }
    }

    pub fn at_pos(pos: Vector3<f32>) -> BodyLocation {
        BodyLocation {
            pos,
            yaw: Deg::zero(),
            pitch: Deg::zero(),
        }
    }

    pub fn rotation_matrix(&self) -> Matrix4<f32> {
        return Matrix4::from_angle_x(self.pitch) * Matrix4::from_angle_y(self.yaw);
    }

    pub fn inv_rotation_matrix(&self) -> Matrix4<f32> {
        return self.rotation_matrix().invert().unwrap();
    }

    pub fn model_to_world_matrix(&self) -> Matrix4<f32> {
        self.rotation_matrix() * Matrix4::from_translation(-self.pos)
    }

    pub fn rotate(&mut self, yaw: Deg<f32>, pitch: Deg<f32>) {
        self.yaw =  (self.yaw + yaw).normalize();
        self.pitch = (self.pitch + pitch).normalize();
    }

    pub fn translate(&mut self, d: Vector3<f32>) {
        self.pos += d;
    }

    pub fn forward(&mut self, dir: Vector3<f32>) {
        let fmax = self.inv_rotation_matrix();
        let forward = fmax * dir.extend(1.0);
        self.translate(forward.truncate());
    }

    pub fn look_at(&mut self, target: Vector3<f32>) {
        let forward: Vector3<f32> = (target - self.pos).normalize();

        let pitch: Deg<f32> = Rad(forward.y.asin()).into();
        let yaw: Deg<f32> = Rad(f32::atan2(forward.x, forward.z)).into();

        self.yaw = yaw;
        self.pitch = pitch;
    }
}


impl Component for BodyLocation {
    type Storage = VecStorage<Self>;
}


#[derive(Debug)]
pub struct Velocity (Vector3<f32>);

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}


pub struct UpdateLocation;

impl<'a> System<'a> for UpdateLocation {
    type SystemData = (
        WriteStorage<'a, BodyLocation>, ReadStorage<'a, Velocity>, Read<'a, DeltaTime>
    );

    fn run(&mut self, (mut loc, vel, delta): Self::SystemData) {
        for (loc, vel) in (&mut loc, &vel).join() {
            let delta_millis = 1.0 / delta.0.as_millis() as f32;
            loc.pos.x += vel.0.x * delta_millis;
            loc.pos.y += vel.0.y * delta_millis;
        }
    }
}
