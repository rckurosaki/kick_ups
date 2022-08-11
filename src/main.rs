use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 60.0;
const BALL_RADIUS: i32 = 1;

struct Ball {
    x: f32,
    y: f32,
    velocity: f32,
}

impl Ball {
    fn new(x: f32, y: f32) -> Self {
        Ball {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(
            self.x as i32,
            self.y as i32,
            YELLOW,
            BLACK,
            to_cp437('O')
        );
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity;
        self.x += 0.0;
        if self.y < 0.0{
            self.y = 0.0;
        }
    }

    fn kick(&mut self) {
        self.velocity = -2.0;
    }
}

struct State {
    ball: Ball,
    frame_time: f32,
    mode: GameMode,
    score: i32,
}

impl State {
    fn new() -> Self {
        State {
            ball: Ball::new(40.0, 25.0),
            frame_time: 0.0,
            mode: GameMode::Menu,
            score: 0,
        }
    }

    fn calculate_hit_box(&mut self, ball: (i32, i32), mouse: (i32, i32), radius:i32) -> bool{
        if (mouse.0 >= ball.0 - radius && mouse.0 <= ball.0 + radius) && (mouse.1 >= ball.1 - radius && mouse.1 <= ball.1 + radius) {
            return true;
        }
        false
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.ball.gravity_and_move();
        }

        if ctx.left_click && self.calculate_hit_box((self.ball.x as i32, self.ball.y as i32), ctx.mouse_pos(), BALL_RADIUS){
            self.ball.kick();
            self.score += 1;
        }
        // if ctx.left_click && ctx.mouse_pos() == (self.ball.x as i32, self.ball.y as i32) {
        //     self.ball.kick();
        //     self.score += 1;
        // }
        self.ball.render(ctx);
        ctx.print(0,0,"Click to kick...");
        if self.ball.y > SCREEN_HEIGHT as f32{
            self.mode = GameMode::End;
        }

        ctx.print(0,1,&format!("Score: {}", self.score));

        // if the ball touches the ground, game over.
        if self.ball.y > SCREEN_HEIGHT as f32 {
            self.mode = GameMode::End;
        }
    }

    fn restart(&mut self) {
        self.ball = Ball::new(40.0, 25.0);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.score = 0; 
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Kick Ups");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, &format!("You earned {} points!", self.score));
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
                        .with_title("Kick Ups")
                        .build()?;

    main_loop(context, State::new())

}
