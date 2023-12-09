use crate::cube::Target;
use ggez::{event::EventHandler, graphics, Context, GameError, GameResult};
use rand::Rng;

pub struct MainState {
    target: Target,
    score: u16,
}

impl MainState {
    // This will organize all the scenary on the screen.
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let height = 20.;
        let width = 20.;
        // TODO: Change this!
        let position_x = 40.;
        let position_y = 40.;
        let state = MainState {
            target: Target::new(height, width, position_x, position_y),
            score: 0,
        };
        Ok(state)
    }

    pub fn state_update(&mut self) {
        self.target.rectangle.x = rand::thread_rng().gen_range(0..1000) as f32;
        self.target.rectangle.y = rand::thread_rng().gen_range(0..700) as f32;
    }

    fn check(&mut self, mouse_button_x: f32, mouse_button_y: f32) -> bool {
        if (self.target.rectangle.x + self.target.rectangle.w >= mouse_button_x)
            && (mouse_button_y <= self.target.rectangle.y + self.target.rectangle.h)
        {
            println!("One check true");
            return true;
        }
        return false;
    }
}

// We have to draw here
impl EventHandler for MainState {
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), GameError> {
        self.state_update();
        if self.check(_x, _y) {
            self.score += 1;
            println!("You scored!")
        }
        Ok(())
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        let dest_point = ggez::glam::Vec2::new(1000., 0.);
        canvas.draw(
            graphics::Text::new(self.score.to_string()).set_scale(48.),
            dest_point,
        );
        self.target.draw(&mut canvas)?;
        canvas.finish(ctx)?;
        Ok(())
    }
}
