mod game;

fn rng() -> u8 {
    7
}

fn btn() -> u8 {
    7
}

fn render(_buffer: &[u8]) {
}

fn reset() {}

fn main() {
    let game = game::GameBus::new(
        rng, btn, render,
        reset
    );
    let mut game = game::Game::new(game);

    loop {
        game.step();
    }
}