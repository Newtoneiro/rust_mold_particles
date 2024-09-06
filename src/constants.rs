pub type GridType = Vec<Vec<f32>>;

pub(crate) mod window_constants {
    pub(crate) const WINDOW_TITLE: &str = "Mold particles";
    pub(crate) const WINDOW_WIDTH: usize = 300;
    pub(crate) const WINDOW_HEIGHT: usize = 300;
}

pub(crate) mod mold_constants {
    pub(crate) const INIT_PARTICLES: usize = 2000;
    pub(crate) const SPAWN_RADIUS: f32 = 70.0;
    pub(crate) const FADE_FACTOR: f32 = 0.01;
    pub(crate) const BLUR_WINDOW_SIZE: i32 = 1;
    pub(crate) const MOVE_SPEED: f32 = 1.0;
    pub(crate) const TURN_SPEED: f32 = 1.0;
    pub(crate) const DELTA_TIME: f32 = 1.0;
    pub(crate) const SENSOR_ANGLE_SPACING: f32 = 50.0;
    pub(crate) const SENSOR_OFFSET_DST: f32 = 10.0;
    pub(crate) const SENSOR_SIZE: i32 = 5;
}
