use cgmath::prelude::*;
use cgmath::{Matrix4, Deg, perspective};
use crate::physics::system::BodyLocation;


pub struct Camera {
    projection_matrix: Matrix4<f32>,

    pub fov: Deg<f32>,
    pub aspect_ratio: f32,
}

impl Camera {
    pub fn new(fov: Deg<f32>, aspect_ratio: f32) -> Camera {
        let mut camera = Camera {
            projection_matrix: Matrix4::identity(),
            fov,
            aspect_ratio,
        };
        camera.rebuild_projection();

        camera
    }

    pub fn rebuild_projection(&mut self) {
        self.projection_matrix = perspective(self.fov, self.aspect_ratio, 0.1, 100.0);
    }

    fn rotation_matrix(&self, yaw: Deg<f32>, pitch: Deg<f32>) -> Matrix4<f32> {
        Matrix4::from_angle_x(pitch) * Matrix4::from_angle_y(yaw)
    }

    pub fn to_matrix(&self, loc: &BodyLocation) -> Matrix4<f32> {
        self.projection_matrix * self.rotation_matrix(loc.yaw, loc.pitch) * Matrix4::from_translation(-loc.pos)
    }
}