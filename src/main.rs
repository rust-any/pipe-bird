use bracket_lib::prelude::*;

// 游戏模式
enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0; // 单位时间，毫秒

struct Bird {
    x: i32, // 向右为正方向
    y: i32, // 向下为正方向
    velocity: f32, // 下降的速度
}

impl Bird {
    fn new(x: i32, y: i32) -> Self {
        Bird { x, y, velocity: 0.0 }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(5, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_effect(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += (self.velocity * 1.0) as i32;
        self.x += 1;

        if self.y < 0 { // 飞到顶端后不再升高
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

struct State {
    mode: GameMode,
    bird: Bird,
    frame_time: f32,
    obstacle: Obstacle,
    score: i32
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            bird: Bird::new(5, 25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0
        }
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
        self.bird = Bird::new(5, 25);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }

    fn menu(&mut self, ctx: &mut BTerm) {
        ctx.cls(); // 清理屏幕
        ctx.print_centered(5, "Welcome to This Game");
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

    fn end(&mut self, ctx: &mut BTerm) {
        ctx.cls(); // 清理屏幕
        ctx.print_centered(5, "Game Over!");
        ctx.print_centered(6, &format!("Score: {}", self.score));
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

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY); // 清理背景并设置颜色
        self.frame_time += ctx.frame_time_ms; // frame_time_ms tick时间间隔？
        if self.frame_time > FRAME_DURATION { // 固定时间间隔下更新重力影响（时间间隔越小，下降越快）
            self.frame_time = 0.0;
            self.bird.gravity_effect();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.bird.flap();
        }

        self.bird.render(ctx);
        ctx.print(0, 0, "Press (Space) to Flap");
        ctx.print(0, 1, &format!("Source: {}", self.score));

        self.obstacle.render(ctx, self.bird.x);

        if self.bird.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.bird.x + SCREEN_WIDTH, self.score);
        }

        if self.bird.y > SCREEN_HEIGHT || self.obstacle.is_hit(&self.bird) {
            self.mode = GameMode::End;
        }
    }

}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.end(ctx),
        }

        // ctx.cls(); // 清理屏幕
        // ctx.print(1, 1, "Hello, world!"); // 打印内容到屏幕
    }
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Self {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score)
        }
    }

    fn render(&mut self, ctx: &mut BTerm, bird_x: i32) {
        let screen_x = self.x - bird_x; // 障碍物在可视区的横坐标
        let half_size = self.size / 2;

        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

    }

    fn is_hit(&self, bird: &Bird) -> bool {
        let half_size = self.size / 2;
        let is_x_match = bird.x == self.x;
        let bird_above_gap = bird.y < self.gap_y - half_size;
        let bird_below_gap = bird.y > self.gap_y + half_size;
        is_x_match && (bird_above_gap || bird_below_gap)
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("pipe-bird")
        .build()?;
    main_loop(context, State::new())
}

