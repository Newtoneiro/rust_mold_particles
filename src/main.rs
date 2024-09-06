mod constants;
mod graphic_controller;
mod map;
mod mold_controller;

use constants::window_constants::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH};
use graphic_controller::GraphicController;
use macroquad::prelude::*;
use mold_controller::MoldController;

fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_string(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        window_resizable: false,
        ..Default::default() // Use default values for other settings
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut gc = GraphicController::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut mc = MoldController::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    mc.init_particles(10000);

    loop {
        gc.update(mc.get_vals_to_update());
        gc.draw_image().await;
        mc.tick();
    }
}
