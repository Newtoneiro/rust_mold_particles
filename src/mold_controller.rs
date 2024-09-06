use rand::{thread_rng, Rng};
use std::f32::consts::PI;

use crate::{
    constants::mold_constants::{
        DELTA_TIME, MOVE_SPEED, SENSOR_ANGLE_SPACING, SENSOR_OFFSET_DST, SENSOR_SIZE, TURN_SPEED,
    },
    map::Map,
};

pub struct MoldParticle {
    pos: (f32, f32),
    angle: f32,
}

pub struct MoldController {
    map: Map,
    particles: Vec<MoldParticle>,
}

impl MoldController {
    pub fn new(width: usize, height: usize) -> Self {
        MoldController {
            map: Map::new(width, height),
            particles: Vec::new(),
        }
    }

    pub fn init_particles(&mut self, n: usize) {
        for _ in 0..n {
            let rot = thread_rng().gen_range(0.0..2.0 * PI);

            self.particles.push(MoldParticle {
                pos: (
                    self.map.width() as f32 / 2.0,
                    self.map.height() as f32 / 2.0,
                ),
                angle: rot,
            })
        }
    }

    fn update_movement(&mut self, idx: usize) {
        let particle: &mut MoldParticle = &mut self.particles[idx];

        let direction = (particle.angle.cos(), particle.angle.sin());

        let mut new_pos = (
            particle.pos.0 as f32 + (direction.0 * DELTA_TIME * MOVE_SPEED),
            particle.pos.1 as f32 + (direction.1 * DELTA_TIME * MOVE_SPEED),
        );

        // Check if the particle hits the boundary of the map (walls)
        let map_width = self.map.width() as f32;
        let map_height = self.map.height() as f32;

        if new_pos.0 < 0.0 || new_pos.0 >= map_width || new_pos.1 < 0.0 || new_pos.1 >= map_height {
            // Change the angle when a collision happens
            if new_pos.0 <= 0.0 || new_pos.0 >= map_width {
                // Left or right wall collision
                // Reverse the horizontal direction
                particle.angle = PI - particle.angle; // Reflect over the vertical axis
            }

            if new_pos.1 <= 0.0 || new_pos.1 >= map_height {
                // Top or bottom wall collision
                // Reverse the vertical direction
                particle.angle = -particle.angle; // Reflect over the horizontal axis
            }

            new_pos.0 = new_pos.0.clamp(0.0, map_width - 0.01);
            new_pos.1 = new_pos.1.clamp(0.0, map_height - 0.01);

            // Add a slight random variation to the angle (±22.5 degrees)
            particle.angle += thread_rng().gen_range(-PI / 4.0..PI / 4.0);

            // Normalize the angle to stay within [0, 2π]
            particle.angle = particle.angle % (2.0 * PI);
            if particle.angle < 0.0 {
                particle.angle += 2.0 * PI;
            }
        }

        particle.pos = new_pos;

        self.map
            .set_field(new_pos.0 as usize, new_pos.1 as usize, 1.0);
    }

    fn get_angle_correction(&self, idx: usize) -> f32 {
        let weight_forward = self.sense(idx, 0.0);
        let weight_left = self.sense(idx, SENSOR_ANGLE_SPACING);
        let weight_right = self.sense(idx, -SENSOR_ANGLE_SPACING);

        let random_steer_strength = thread_rng().gen_range(0.0..1.0);

        if weight_forward > weight_left && weight_forward > weight_right {
            return 0.0;
        } else if weight_forward < weight_left && weight_forward < weight_right {
            return (random_steer_strength - 0.5) * 2.0 * TURN_SPEED * DELTA_TIME;
        } else if weight_right > weight_left {
            return -random_steer_strength * TURN_SPEED * DELTA_TIME;
        } else if weight_left > weight_right {
            return random_steer_strength * TURN_SPEED * DELTA_TIME;
        }

        0.0
    }

    fn sense(&self, idx: usize, sensor_angle_offset: f32) -> f32 {
        let particle = &self.particles[idx];

        let sensor_angle = particle.angle + sensor_angle_offset;
        let sensor_dir = (sensor_angle.cos(), sensor_angle.sin());
        let sensor_centre = (
            particle.pos.0 + sensor_dir.0 * SENSOR_OFFSET_DST,
            particle.pos.1 + sensor_dir.1 * SENSOR_OFFSET_DST,
        );
        let mut sum = 0.0;

        for offset_x in -SENSOR_SIZE..SENSOR_SIZE {
            for offset_y in -SENSOR_SIZE..SENSOR_SIZE {
                let pos = (
                    sensor_centre.0 + offset_x as f32,
                    sensor_centre.1 + offset_y as f32,
                );

                if pos.0 >= 0.0
                    && pos.0 < self.map.width() as f32
                    && pos.1 >= 0.0
                    && pos.1 < self.map.height() as f32
                {
                    sum += self.map.get_field(pos.0 as usize, pos.1 as usize);
                }
            }
        }

        sum
    }

    pub fn get_vals_to_update(&self) -> Vec<((i32, i32), f32)> {
        self.map.get_vals_to_update()
    }

    pub fn tick(&mut self) {
        for idx in 0..self.particles.len() {
            self.update_movement(idx);
            self.particles[idx].angle += self.get_angle_correction(idx);
        }
        self.map.blur();
        self.map.fade();
    }
}
