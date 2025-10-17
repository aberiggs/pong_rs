use ggez::{
    Context, GameResult,
    event::EventHandler,
    graphics::{self, Color, Rect},
    input::keyboard::{KeyCode, KeyboardContext},
    mint::Point2,
};

/// Constants
const PADDLE_SPEED: f32 = 5.0;
const BALL_SPEED: f32 = 8.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 100.0;
const BALL_RADIUS: f32 = 8.0;

pub struct GameState {
    left_paddle_pos: Point2<f32>,
    right_paddle_pos: Point2<f32>,
    ball_pos: Point2<f32>,
    ball_vel: Point2<f32>,
    score: (u8, u8), // Play until this overflows ^.^
}

impl GameState {
    /// Generates a random ball velocity with a random angle
    fn random_ball_velocity() -> Point2<f32> {
        let angle = rand::random::<f32>() * std::f32::consts::PI;
        Point2 {
            x: angle.cos() * BALL_SPEED,
            y: angle.sin() * BALL_SPEED,
        }
    }

    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        ctx.gfx.set_window_title("Rusty Pong");
        let (width, height) = ctx.gfx.drawable_size();

        Ok(GameState {
            left_paddle_pos: Point2 {
                x: 20.,
                y: (height / 2.) - (PADDLE_HEIGHT / 2.),
            },
            right_paddle_pos: Point2 {
                x: width - PADDLE_WIDTH - 20.,
                y: (height / 2.) - (PADDLE_HEIGHT / 2.),
            },
            ball_pos: Point2 {
                x: (width / 2.),
                y: (height / 2.),
            },
            ball_vel: Self::random_ball_velocity(),
            score: (0, 0),
        })
    }

    /// Takes the keyboard context and handles the input.
    fn handle_input(&mut self, keyboard: &KeyboardContext) -> GameResult {
        let mut pos_change = 0.;

        for key in keyboard.pressed_keys() {
            match key {
                // Note: origin is TLC
                KeyCode::W => pos_change = -1.,
                KeyCode::S => pos_change = 1.,
                _ => {} // Ignore other keys
            }
        }

        self.left_paddle_pos.y += pos_change * PADDLE_SPEED;
        self.right_paddle_pos.y += pos_change * PADDLE_SPEED;

        Ok(())
    }

    /// Handles the ball movement.
    /// Ball moves and may collide with the paddles.
    fn handle_ball_movement(&mut self, y_bound: f32) -> GameResult {
        // Do basic movement
        self.ball_pos.x += self.ball_vel.x;
        self.ball_pos.y += self.ball_vel.y;

        let speed = (self.ball_vel.x * self.ball_vel.x + self.ball_vel.y * self.ball_vel.y).sqrt();

        let left_center_y = self.left_paddle_pos.y + (PADDLE_HEIGHT / 2.);
        let left_surface_x = self.left_paddle_pos.x + PADDLE_WIDTH;

        // Handle collisions with the left paddle
        if self.ball_pos.x - BALL_RADIUS < left_surface_x
            && self.ball_pos.x - BALL_RADIUS > left_surface_x - PADDLE_WIDTH
        {
            let distance_from_center = self.ball_pos.y - left_center_y;
            let angle = distance_from_center / (PADDLE_HEIGHT / 2.);
            if angle.abs() <= 1. {
                self.ball_vel.x = angle.cos() * speed;
                self.ball_vel.y = angle.sin() * speed;
            }
        }

        let right_center_y = self.right_paddle_pos.y + (PADDLE_HEIGHT / 2.);
        let right_surface_x = self.right_paddle_pos.x;

        // Handle collisions with the right paddle
        if self.ball_pos.x + BALL_RADIUS > right_surface_x
            && self.ball_pos.x + BALL_RADIUS < right_surface_x + PADDLE_WIDTH
        {
            let distance_from_center = self.ball_pos.y - right_center_y;

            let angle = distance_from_center / (PADDLE_HEIGHT / 2.);
            if angle.abs() <= 1. {
                // Flip x velocity since it's coming from the right
                self.ball_vel.x = angle.cos() * -speed;
                self.ball_vel.y = angle.sin() * speed;
            }
        }

        // Handle collisions with horizontal walls
        if self.ball_pos.y - BALL_RADIUS < 0. || self.ball_pos.y + BALL_RADIUS > y_bound {
            self.ball_vel.y = -self.ball_vel.y;
        }

        Ok(())
    }

    fn handle_potential_score(&mut self, x_bound: f32) -> bool {
        let (left_score, right_score) = &mut self.score;

        if self.ball_pos.x - BALL_RADIUS < 0. {
            *right_score += 1;
        } else if self.ball_pos.x + BALL_RADIUS > x_bound {
            *left_score += 1;
        } else {
            return false;
        }

        return true;
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.handle_input(&ctx.keyboard)?;

        let (width, height) = ctx.gfx.drawable_size();
        self.handle_ball_movement(height)?;
        if self.handle_potential_score(width) {
            // Reset ball position and velocity
            self.ball_pos = Point2 {
                x: width / 2.,
                y: height / 2.,
            };
            self.ball_vel = Self::random_ball_velocity();

            // Reset paddles position
            self.left_paddle_pos = Point2 {
                x: 20.,
                y: (height / 2.) - (PADDLE_HEIGHT / 2.),
            };
            self.right_paddle_pos = Point2 {
                x: width - PADDLE_WIDTH - 20.,
                y: (height / 2.) - (PADDLE_HEIGHT / 2.),
            };
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // Set up meshes
        let left_paddle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(
                self.left_paddle_pos.x,
                self.left_paddle_pos.y,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
            Color::WHITE,
        )?;

        let right_paddle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(
                self.right_paddle_pos.x,
                self.right_paddle_pos.y,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
            Color::WHITE,
        )?;

        let ball = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Point2 {
                x: self.ball_pos.x,
                y: self.ball_pos.y,
            },
            BALL_RADIUS,
            3.0,
            Color::WHITE,
        )?;

        // Draw meshes
        canvas.draw(&left_paddle, graphics::DrawParam::new());
        canvas.draw(&right_paddle, graphics::DrawParam::new());
        canvas.draw(&ball, graphics::DrawParam::new());

        // Draw score
        let score_text = graphics::Text::new(format!("{} - {}", self.score.0, self.score.1));
        let (width, _height) = ctx.gfx.drawable_size();
        canvas.draw(
            &score_text,
            graphics::DrawParam::new()
                .dest(Point2 {
                    x: width / 2.0 - 20.0,
                    y: 20.0,
                })
                .color(Color::WHITE),
        );

        canvas.finish(ctx)?;

        Ok(())
    }
}
