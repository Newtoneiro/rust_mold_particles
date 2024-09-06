pub type GridType = Vec<Vec<f32>>;

pub(crate) mod window_constants {
    pub(crate) const WINDOW_TITLE: &str = "Mold particles";
    pub(crate) const WINDOW_WIDTH: usize = 600;
    pub(crate) const WINDOW_HEIGHT: usize = 600;
}

pub(crate) mod mold_constants {
    pub(crate) const FADE_FACTOR: f32 = 0.008;
    pub(crate) const BLUR_WINDOW_SIZE: i32 = 1;
    pub(crate) const MOVE_SPEED: f32 = 1.0;
    pub(crate) const DELTA_TIME: f32 = 1.0;
    pub(crate) const SENSOR_OFFSET_DST: f32 = 1.0;
    pub(crate) const SENSOR_SIZE: i32 = 20;
}
