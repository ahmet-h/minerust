use std::ops::Add;

use glam::{vec3, Mat4, Vec3};

pub struct Camera {
    pos: Vec3,
    front: Vec3,
    up: Vec3,
    right: Vec3,
    world_up: Vec3,
    yaw: f32,
    pitch: f32,
    fov: f32,
    near: f32,
    far: f32,
    speed: f32,
    move_dir: Vec3,
    max_speed: f32,

    view: Mat4,
    projection: Mat4,
    projection_view: Mat4,
}

impl Camera {
    pub fn new(pos: Vec3) -> Self {
        let mut camera = Self {
            pos,
            ..Default::default()
        };
        camera.update();
        camera
    }

    pub fn add_yaw(&mut self, x: f32) {
        self.yaw += x.clamp(-180., 180.);
        if self.yaw > 180. {
            self.yaw -= 360.;
        } else if self.yaw <= -180. {
            self.yaw += 360.;
        }
    }

    pub fn add_pitch(&mut self, y: f32) {
        self.pitch += y;
        if self.pitch >= 89. {
            self.pitch = 89.;
        } else if self.pitch <= -89. {
            self.pitch = -89.;
        }
    }

    pub fn pos(&self) -> Vec3 {
        self.pos
    }

    pub fn front(&self) -> Vec3 {
        self.front
    }

    pub fn right(&self) -> Vec3 {
        self.right
    }

    pub fn update(&mut self) {
        let mut front = vec3(0., 0., 0.);
        front.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        front.y = self.pitch.to_radians().sin();
        front.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();
        self.front = front;

        self.right = self.front.cross(self.world_up).normalize();
        self.up = self.right.cross(self.front).normalize();
    }

    fn get_view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.pos, self.pos.add(self.front), self.up)
    }

    fn get_projection_matrix(&self, aspect_ratio: f32) -> Mat4 {
        Mat4::perspective_rh_gl(self.fov.to_radians(), aspect_ratio, self.near, self.far)
    }

    pub fn update_view_matrix(&mut self) {
        self.view = self.get_view_matrix();
    }

    pub fn update_projection_matrix(&mut self, aspect_ratio: f32) {
        self.projection = self.get_projection_matrix(aspect_ratio);
    }

    pub fn update_projection_view_matrix(&mut self, aspect_ratio: f32) {
        self.update_view_matrix();
        self.update_projection_matrix(aspect_ratio);
        self.projection_view = self.projection * self.view;
    }

    pub fn view(&self) -> Mat4 {
        self.view
    }

    pub fn projection(&self) -> Mat4 {
        self.projection
    }

    pub fn projection_view(&self) -> Mat4 {
        self.projection_view
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: vec3(0., 0., -3.),
            front: vec3(0., 0., 1.),
            up: vec3(0., 1., 0.),
            right: vec3(-1., 0., 0.),
            world_up: vec3(0., 1., 0.),
            yaw: 90.,
            pitch: 0.,
            fov: 42.,
            near: 0.5,
            far: 400.,
            speed: 0.,
            move_dir: vec3(0., 0., 0.),
            max_speed: 5.0,

            projection: Default::default(),
            view: Default::default(),
            projection_view: Default::default(),
        }
    }
}