// Could be a rectangle or a cube
use ggez::graphics;
use ggez::graphics::Color;
pub struct Target {
    pub rectangle: graphics::Rect,
}

impl Target {
    pub fn new(height: f32, width: f32, position_x: f32, position_y: f32) -> Target {
        Target {
            rectangle: graphics::Rect {
                x: position_x,
                y: position_y,
                w: width,
                h: height,
            },
        }
    }

    // For drawing the target of any shape on the screen
    pub fn draw(&self, canvas: &mut graphics::Canvas) -> ggez::GameResult<()> {
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(self.rectangle.point())
                .color(Color::WHITE)
                .scale(self.rectangle.size()),
        );
        Ok(())
    }
}
