use ggez::{
    Context,
    ContextBuilder,
    GameResult,
    GameError,
    timer,
    conf::{WindowSetup, WindowMode},
    event::{self, EventHandler},
    input::keyboard::{self, KeyCode},
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
use rand::prelude::*;

type Point = mint::Point2<f32>;

#[derive(Copy, Clone, Debug, PartialEq)]
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
    fruit: Point,
}

impl Game {
    fn new(_ctx: &mut Context, conf: GameConf) -> Self {
        Self {
            conf,
            snake: Snake::new(new_point(
                (conf.grid_size.x / 2.).floor(),
                (conf.grid_size.y / 2.).floor(),
            )),
            fruit: random_point(conf.grid_size),
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

    fn draw_fruit(&mut self, ctx: &mut Context) -> GameResult {
        self.draw_square(ctx, self.fruit, Color::RED)
    }

    fn new_fruit(&mut self) {
        let mut rng = rand::thread_rng();
        let mut points = vec![];
        for i in 0..self.conf.grid_size.y as i32 {
            for j in 0..self.conf.grid_size.x as i32 {
                points.push(new_point(j as f32, i as f32));
            }
        }
        self.fruit = points[rng.gen_range(0..points.len())];
    }
}

impl EventHandler<GameError> for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if !self.snake.alive {
            event::quit(ctx);
        } else if timer::check_update_time(ctx, self.conf.fps) {
            self.snake.update(self.conf.grid_size)?;
            if self.snake.head == self.fruit {
                self.snake.length_to_add += 2;
                self.new_fruit();
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
        self.draw_grid(ctx)?;
        self.draw_snake(ctx)?;
        self.draw_fruit(ctx)?;
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, _mods: keyboard::KeyMods, _: bool) {
        self.snake.handle_kbin(key).unwrap();
    }
}

#[derive(Copy, Clone)]
struct GameConf {
    window_size: Point,
    grid_size: Point,
    cell_size: f32,
    fps: u32,
}

impl GameConf {
    fn new(grid_size: Point, cell_size: f32, fps: u32) -> Self {
        Self {
            window_size: new_point(grid_size.x * cell_size, grid_size.y * cell_size),
            grid_size,
            cell_size,
            fps,
        }
    }

}

struct Snake {
    head: Point,
    tail: VecDeque<Point>,
    length_to_add: i32,
    direction: Direction,
    alive: bool,
    input_buffer: VecDeque<Direction>,
    input_buffer_limit: usize,
}

impl Snake {
    fn new(point: Point) -> Self {
        Self {
            head: point,
            tail: VecDeque::new(),
            length_to_add: 3,
            direction: Direction::Up,
            alive: true,
            input_buffer: VecDeque::new(),
            input_buffer_limit: 3,
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
            if let Some(direction) = self.input_buffer.pop_front() {
                if direction != self.direction.opposite() {
                    self.direction = direction;
                }
            }
        }
        Ok(())
    }

    fn handle_kbin(&mut self, key: KeyCode) -> GameResult {
        let direction = match key {
            KeyCode::Up | KeyCode::W => Some(Direction::Up),
            KeyCode::Down | KeyCode::S => Some(Direction::Down),
            KeyCode::Left | KeyCode::A => Some(Direction::Left),
            KeyCode::Right | KeyCode::D => Some(Direction::Right),
            _ => None,
        };
        if let Some(direction) = direction {
            if self.input_buffer.len() < self.input_buffer_limit {
                self.input_buffer.push_back(direction);
            }
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

fn random_point(range: Point) -> Point {
    let mut rng = rand::thread_rng();
    new_point(
        rng.gen_range(0.0..range.x).floor(),
        rng.gen_range(0.0..range.y).floor(),
    )
}

fn main() -> GameResult{
    let conf = GameConf::new(new_point(65., 45.), 40., 10);
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
