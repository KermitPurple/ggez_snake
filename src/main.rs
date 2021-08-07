use ggez::{
    Context,
    ContextBuilder,
    GameResult,
    GameError,
    conf::{WindowSetup, WindowMode},
    event::{self, EventHandler},
    graphics::{
        self,
        mint,
        Font,
        Text,
        Rect,
        Color,
        DrawMode,
        DrawParam,
    },
};
use winit::dpi::PhysicalPosition;

type Point = mint::Point2<f32>;

struct Game {
    conf: GameConf,
}

impl Game {
    fn new(_ctx: &mut Context, conf: GameConf) -> Self {
        Self {
            conf,
        }
    }
}

impl EventHandler<GameError> for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
        graphics::present(ctx)
    }
}

struct GameConf {
    window_size: Point,
    grid_size: Point,
    cell_size: f32,
}

impl GameConf {
    fn new(grid_size: Point, cell_size: f32) -> Self {
        Self {
            window_size: new_point(grid_size.x * cell_size, grid_size.y * cell_size),
            grid_size,
            cell_size,
        }
    }

}

fn center_rect(w: f32, h: f32) -> Rect {
    Rect {
        x: -w / 2.,
        y: -h / 2.,
        w,
        h,
    }
}

fn center_square(size: f32) -> Rect {
    center_rect(size, size)
}

fn new_point(x: f32, y: f32) -> Point {
    Point {
        x,
        y,
    }
}

fn main() -> GameResult{
    let conf = GameConf::new(new_point(65., 45.), 40.);
    let (mut ctx, event_loop) = ContextBuilder::new("Snake", "KermitPurple")
        .window_setup(WindowSetup{
            title: String::from("Snake"),
            ..Default::default()
        })
        .window_mode(WindowMode{
            width: conf.window_size.x,
            height: conf.window_size.y,
            ..Default::default()
        })
        .build()?;
    let game = Game::new(&mut ctx, conf);
    graphics::set_window_position(&ctx, PhysicalPosition::new(20, 20))?;
    event::run(ctx, event_loop, game);
}
