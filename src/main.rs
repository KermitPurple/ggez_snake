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
use std::collections::VecDeque;

type Point = mint::Point2<f32>;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn opposite(&self) -> Direction {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

struct Game {
    conf: GameConf,
    snake: Snake,
}

impl Game {
    fn new(_ctx: &mut Context, conf: GameConf) -> Self {
        Self {
            conf,
            snake: Snake::new(new_point(
                (conf.grid_size.x / 2.).floor(),
                (conf.grid_size.y / 2.).floor(),
            )),
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

    fn draw_snake(&mut self, ctx: &mut Context) -> GameResult {
        self.draw_square(ctx, self.snake.head, Color::from_rgb(119, 199, 120))?;
        for tail in self.snake.tail.clone() {
            self.draw_square(ctx, tail, Color::from_rgb(119, 177, 120))?;
        }
        Ok(())
    }
}

impl EventHandler<GameError> for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.snake.update(self.conf.grid_size)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
        self.draw_grid(ctx)?;
        self.draw_snake(ctx)?;
        graphics::present(ctx)
    }
}

#[derive(Copy, Clone)]
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

struct Snake {
    head: Point,
    tail: VecDeque<Point>,
    length_to_add: i32,
    direction: Direction,
    alive: bool,
}

impl Snake {
    fn new(point: Point) -> Self {
        Self {
            head: point,
            tail: VecDeque::new(),
            length_to_add: 3,
            direction: Direction::Up,
            alive: true,
        }
    }

    fn update_tail(&mut self) -> GameResult {
        self.tail.push_front(self.head);
        if self.length_to_add <= 0 {
            let _ = self.tail.pop_back();
        } else {
            self.length_to_add -= 1;
        }
        Ok(())
    }

    fn update(&mut self, grid_size: Point) -> GameResult {
        if !self.alive {
            return Ok(());
        }
        let new_head = match self.direction {
            Direction::Up => new_point(self.head.x, self.head.y - 1.),
            Direction::Down => new_point(self.head.x, self.head.y + 1.),
            Direction::Left => new_point(self.head.x - 1., self.head.y),
            Direction::Right => new_point(self.head.x + 1., self.head.y),
        };
        if new_head.x < 0. ||
            new_head.y < 0. ||
            new_head.x >= grid_size.x ||
            new_head.y >= grid_size.y ||
            self.tail.contains(&new_head) {
            self.alive = false;
        } else {
            self.update_tail()?;
            self.head = new_head;
        }
        Ok(())
    }
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
    event::run(ctx, event_loop, game)
}
