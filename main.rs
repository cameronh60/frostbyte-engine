use std::time::{Duration, Instant};

struct Game {
    start_time: Instant,
}

impl Game {
    fn new() -> Self {
        Game {
            start_time: Instant::now(),
        }
    }

    fn update(&self) {
        let elapsed = self.start_time.elapsed();
        println!("Game running for: {:?}", elapsed);
    }
}

fn main() {
    let game = Game::new();
    std::thread::sleep(Duration::from_secs(2));
    game.update();
}
