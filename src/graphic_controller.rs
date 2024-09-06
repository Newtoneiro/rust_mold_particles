use macroquad::prelude::*;

pub struct GraphicController {
    image: Image,
}

impl GraphicController {
    pub fn new(width: usize, height: usize) -> Self {
        GraphicController {
            image: GraphicController::init_image(width, height),
        }
    }

    pub fn init_image(width: usize, height: usize) -> Image {
        let mut img = Image::gen_image_color(width as u16, height as u16, BLACK);

        // Fill the image with grayscale values
        for y in 0..width {
            for x in 0..height {
                img.set_pixel(x as u32, y as u32, Color::from_rgba(0, 0, 0, 255));
            }
        }

        img
    }

    pub fn update(&mut self, values: Vec<((i32, i32), f32)>) {
        for ((x, y), val) in values {
            let color_value = (val * 255.0) as u8;
            let color = Color::from_rgba(color_value, color_value, color_value, 255);
            self.image.set_pixel(x as u32, y as u32, color);
        }
    }

    pub async fn draw_image(&self) {
        let texture = Texture2D::from_image(&self.image);

        clear_background(BLACK);

        // Draw the texture
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            GREEN,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // End the frame
        next_frame().await;
    }
}
