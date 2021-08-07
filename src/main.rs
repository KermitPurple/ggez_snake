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

    fn draw_grid(&mut self, ctx: &mut Context) -> GameResult {
        let mut mb = graphics::MeshBuilder::new();
        let x = self.conf.grid_size.x as i32;
        let y = self.conf.grid_size.y as i32;
        let ws = self.conf.window_size;
        let c = self.conf.cell_size;
        let color = Color::from_rgb(20, 20, 20);
        for i in 0..x {
            let x = i as f32 * c;
            mb.line(&[new_point(x, 0.), new_point(x, ws.y)], 1., color)?;
        }
        for i in 0..y {
            let y = i as f32 * c;
            mb.line(&[new_point(0., y), new_point(ws.x, y)], 1., color)?;
        }
        let mesh = mb.build(ctx)?;
        graphics::draw(ctx, &mesh, DrawParam::default())
    }

    fn draw_square(&mut self, ctx: &mut Context, point: Point, color: Color) -> GameResult {
        let c = self.conf.cell_size;
        let square = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(0., 0., c, c), color)?;
        let point = new_point(
            point.x * c,
            point.y * c,
        );
        graphics::draw(ctx, &square, DrawParam::default().dest(point))
    }
}

impl EventHandler<GameError> for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
        self.draw_grid(ctx)?;
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
