extern crate sfml;
use sfml::{
    graphics::*,
    window::*,
    system::*
};

mod engine;

fn main() {

    let mut window = RenderWindow::new(
        (1690, 940),
        "SlimeJump",
        Style::DEFAULT,
        &Default::default()
    );
    window.set_vertical_sync_enabled(true);

    let frank = Texture::from_file("src/Bee.png").unwrap();

    let font = Font::from_file("src/assets/Fonts/vcr.ttf").unwrap();

    let mut sprite = Sprite::new();
    sprite.set_texture(&frank, true);
    sprite.set_position((400.0, 300.0));

    let mut title = Text::new("Test!", &font, 50);
    title.set_position((600.0, 300.0));

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed {
                    code: Key::ESCAPE, ..
                } => return,
                Event::KeyPressed { code, .. } => {
                    if code == Key::W {
                        sprite.move_((0.0, -10.0));
                    } else if code == Key::S {
                        sprite.move_((0.0, 10.0));
                    } else if code == Key::A {
                        sprite.move_((-10.0, 0.0));
                    } else if code == Key::D {
                        sprite.move_((10.0, 0.0));
                    }
                },
                _ => {}


            }
        }
        window.clear(Color::BLACK);
        window.draw(&sprite);
        window.draw(&title);
        window.display();
    }
}
