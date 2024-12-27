#![allow(dead_code)]
use std::{collections::HashMap, hash::Hash};

// use winit::event::VirtualKeyCode;
use super::{Vector3, Matrix3, Matrix4};


pub struct Camera<MovementControlVariable: Eq + Hash> {
    pub position: Vector3, 
    pub direction: Vector3,
    pub up: Vector3,
    pub move_speed: f32,
    pub rotate_speed: f32,
    pub movement: [bool; 10], // forward, back, left, right, up, down, spin right, spin left, spin forward, spin backward
    is_controlled: bool,


    pub movement_map: HashMap<MovementControlVariable, CameraDirections>
}




pub enum CameraDirections {
    Forward,
    Backwards,
    Left,
    Right,
    Up,
    Down,
    SpinRight,
    SpinLeft,
    SpinForward,
    SpinBackward
}

impl<T: Eq + Hash> Camera<T> {

    pub fn new(start_pos: Option<[f32; 3]>, start_dir: Option<[f32; 3]>, move_speed: Option<f32>, rotate_speed: Option<f32>, movement_map: Option<HashMap<T, CameraDirections>>) -> Self{
        let position: Vector3 = {
            if start_pos.is_some() {
                start_pos.unwrap().into()
            } else {
                Vector3::ZERO
            }
        };

        let direction: Vector3 = {
            if start_dir.is_some() && start_dir.unwrap() != [0.0; 3] {
                start_dir.unwrap().into()
            } else {
                Vector3::X
            }
        };

        let control_map = if movement_map.is_some() {movement_map.unwrap()} else {HashMap::new()};

        Camera {
            position,
            direction,
            move_speed: move_speed.unwrap_or(3.0),
            rotate_speed: rotate_speed.unwrap_or(1.0),
            movement: [false; 10],
            up: -Vector3::Y,
            is_controlled: false,
            movement_map: control_map
        }
    }

    pub fn controllable(&mut self) {
        self.is_controlled = true;
    }

    pub fn toggle_controlled(&mut self) {
        self.is_controlled ^= true;
    }

    pub fn get_view_matrix(&self) -> Matrix4 {
        let f = self.direction.normalised();
    
        let mut s = self.up.cross(f);

        s.normalise();
    
        let u = f.cross(s);
    
        Matrix4::new(
            s.x, u.x, -f.x, 0.0,
            s.y, u.y, -f.y, 0.0,
            s.z, u.z, -f.z, 0.0,
            -self.position.dot(s), -self.position.dot(u), self.position.dot(f), 1.0
        )
    }

    pub fn look_at(&mut self, target: Vector3) {
        self.direction = (target - self.position).normalised();
    }

    pub fn process_input(&mut self, control: T, state: bool) {
        match self.movement_map.get(&control) {
            Some(CameraDirections::Forward) => {
                self.movement[0] = state;
            }
            Some(CameraDirections::Backwards) => {
                self.movement[1] = state;
            }
            Some(CameraDirections::Left) => {
                self.movement[2] = state;
            }
            Some(CameraDirections::Right) => {
                self.movement[3] = state;
            }
            Some(CameraDirections::Up) => {
                self.movement[4] = state;
            }
            Some(CameraDirections::Down) => {
                self.movement[5] = state;
            }
            Some(CameraDirections::SpinLeft) => {
                self.movement[6] = state;
            }
            Some(CameraDirections::SpinRight) => {
                self.movement[7] = state;
            }
            Some(CameraDirections::SpinForward) => {
                self.movement[8] = state;
            }
            Some(CameraDirections::SpinBackward) => {
                self.movement[9] = state;
            }
            _ => ()
        }
    }




    pub fn do_move(&mut self, delta_time: f32) {
        if !self.is_controlled {return;}

        // take cross of direction and up to get left
        let mut left = self.direction.cross(self.up);

        let forward = left.cross(self.up);
        // forward/back
        if self.movement[0] {self.position -= forward * self.move_speed * delta_time}
        if self.movement[1] {self.position += forward * self.move_speed * delta_time}
        // left/right
        if self.movement[2] {self.position += left * self.move_speed * delta_time}
        if self.movement[3] {self.position -= left * self.move_speed * delta_time}
        // up/down
        if self.movement[4] {self.position -= self.up * self.move_speed * delta_time}
        if self.movement[5] {self.position += self.up * self.move_speed * delta_time}

        // spin around up
        // normalise up
        self.up.normalise();
        // rotate
        if self.movement[6] {
            let rotation = Matrix3::from_angle_and_axis(self.rotate_speed * delta_time, self.up);
            self.direction = rotation * self.direction;
        }
        if self.movement[7] {
            let rotation = Matrix3::from_angle_and_axis(-self.rotate_speed * delta_time, self.up);
            self.direction = rotation * self.direction;
        }

        // spin around left
        // normalise left
        left.normalise();
        // rotate
        if self.movement[8] {
            let rotation = Matrix3::from_angle_and_axis(self.rotate_speed * delta_time, left);
            self.direction = rotation * self.direction;
        }
        if self.movement[9] {
            let rotation = Matrix3::from_angle_and_axis(-self.rotate_speed * delta_time, left);
            self.direction = rotation * self.direction;
        }
    }
}

