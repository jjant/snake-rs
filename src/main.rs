use runty8::{self, App, Button, Pico8};

#[derive(Default)]
struct Game {
    player: Player,
    apples: Vec<Apple>,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn to_delta(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, 1),
            Direction::West => (1, 0),
            Direction::South => (0, -1),
            Direction::East => (-1, 0),
        }
    }
}

struct Player {
    x: i32,
    y: i32,
    direction: Direction,
    move_cooldown_max: u32,
    move_cooldown: u32,
}

impl Player {
    fn new() -> Self {
        let move_cooldown_max = 30;

        Self {
            x: 1,
            y: 1,
            direction: Direction::West,
            move_cooldown_max,
            move_cooldown: move_cooldown_max,
        }
    }

    fn update(&mut self, direction: Direction) {
        self.direction = direction;

        self.move_cooldown -= 1;
        if self.move_cooldown <= 0 {
            let (delta_x, delta_y) = self.direction.to_delta();
            self.x += delta_x;
            self.y += delta_y;
            self.move_cooldown = self.move_cooldown_max;
        }

        self.x = runty8::mid(0.0, self.x as f32, 14.0) as i32;
        self.y = runty8::mid(0.0, self.y as f32, 14.0) as i32;
    }

    fn draw(&mut self, pico8: &mut runty8::Pico8) {
        pico8.spr(1, (self.x + 1) * 8, 128 - (self.y + 1) * 8);
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl App for Game {
    fn init(_: &mut Pico8) -> Self {
        let mut game = Self::default();
        game.apples.push(Apple::random());
        game
    }

    fn update(&mut self, pico8: &mut runty8::Pico8) {
        let mut direction = self.player.direction;

        if pico8.btn(Button::Right) {
            direction = Direction::West;
        }
        if pico8.btn(Button::Left) {
            direction = Direction::East;
        }
        if pico8.btn(Button::Up) {
            direction = Direction::North;
        }
        if pico8.btn(Button::Down) {
            direction = Direction::South;
        }

        println!("{:?}", direction);
        self.player.update(direction);
    }

    fn draw(&mut self, pico8: &mut runty8::Pico8) {
        pico8.cls(0);
        draw_background(pico8);
        self.apples.iter().for_each(|apple| apple.draw(pico8));
        self.player.draw(pico8);
    }
}

fn draw_background(pico8: &mut runty8::Pico8) {
    for y in 0..16 {
        pico8.spr(2, 0, y * 8);
        pico8.spr(2, 8 * 15, y * 8);
    }

    for x in 0..16 {
        pico8.spr(2, x * 8, 0);
        pico8.spr(2, x * 8, 8 * 15);
    }
}

fn main() {
    let resources = runty8::load_assets!("./assets").unwrap();

    runty8::run_editor::<Game>(resources).unwrap();
}

struct Apple {
    x: i32,
    y: i32,
}

impl Apple {
    fn random() -> Self {
        Self {
            x: runty8::rnd(15.0).floor() as i32,
            y: runty8::rnd(15.0).floor() as i32,
        }
    }

    fn draw(&self, pico8: &mut Pico8) {
        pico8.spr(4, self.x * 8, 128 - self.y * 8);
    }
}
