use sdl2::{pixels::Color, rect::{Point, Rect}, video::WindowContext};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use std::time::Duration;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::render::TextureCreator;
use entities::*;

// https://sunjay.dev/learn-game-dev/intro.html, learning game dev

mod entities;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem.window("game", 800, 600)
        .position_centered()
        .build()
        .expect("failed to build window");
    let mut canvas = window.into_canvas().build()
        .expect("Failed to build canvas");
    let texture_creator = canvas.texture_creator();
    let (mut player_list, mut player) = create_entities(&texture_creator);
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
	for event in event_pump.poll_iter() {
	    match event {
		Event::Quit {..} |
		Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
		    break 'running;
		},
		Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    player.velocity.direction = Direction::Left;
		    player.velocity.current = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
		    player.velocity.direction = Direction::Right;
		    player.velocity.current = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    player.velocity.direction = Direction::Up;
		    player.velocity.current = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
		    player.velocity.direction = Direction::Down;
		    player.velocity.current = true;
                },
		Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
		Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
		Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
		Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
		    player.velocity.current = false;
		}
		_ => {}
	    }
	}
	// game loop
	// update function
	
	Player::update_player(&mut player);
	
	// render function
	render(&mut canvas, Color::RGB(0, 0, 0), &mut player_list, &player)?;
	// time management
	std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20 /* fps */));
    }
    Ok(())
}

fn render(canvas: &mut WindowCanvas, color: Color, player_list: &mut Vec<Player>, local_player: &Player ) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();
    let _ = entity_render_loop(canvas, player_list, local_player);
    canvas.present();
    Ok(())
}

fn entity_render_loop(canvas: &mut WindowCanvas , player_list: &Vec<Player>, local_player: &Player) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;
    // treat center of screen as 0,0 coords
    let screen_position = local_player.pos + Point::new(width as i32 / 2, height as i32 / 2 );
    let screen_rect = Rect::from_center(screen_position, local_player.sprite.size.width() * 2, local_player.sprite.size.height() * 2);
    canvas.copy(&local_player.sprite.texture, local_player.sprite.size, screen_rect)?;
    for player in player_list {
	let screen_position = player.pos + Point::new(width as i32 / 2, height as i32 / 2 );
	let screen_rect = Rect::from_center(screen_position, player.sprite.size.width() * 2, player.sprite.size.height() * 2);
	canvas.copy(&player.sprite.texture, player.sprite.size, screen_rect)?;
    }
    Ok(())
}

fn create_entities<'a>(texture_creator: &'a TextureCreator<WindowContext>) -> (Vec<Player<'a>>, Player<'a>) {
    let texture = texture_creator.load_texture("c:/Users/black/projects/rust/RealVampire/assets/reaper.png")
	.expect("failed to load texture");
    let player_list = Vec::new();
    let movement_struct = Velocity {
	direction: Direction::Down,
	current: false,
	speed: 5,
	previous_direction: Direction::Down,
    };
    let frame = Frame {
	direction: 0,
	current_frame: 0,
    };
    let sprite = Sprite {
	texture: texture,
	size: Rect::new(0,0,26,36),
    };
    let player = Player::new(Point::new(50,50), sprite, movement_struct, frame);
    return (player_list, player)
}
