use sdl2::{pixels::Color, rect::{Point, Rect}, render::Texture};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use std::time::Duration;
use sdl2::image::{self, LoadTexture, InitFlag};

// https://sunjay.dev/learn-game-dev/intro.html, learning game dev

struct Player<'a> {
    pos: Point,
    texture: Texture<'a>,
    sprite: Rect,
    moving: Movement,
    frame: Frame,
}

#[derive(Default, Clone, Copy)]
struct Movement {
    direction: Direction,
    current: bool,
    speed: i32,
    previous_direction: Direction, 
}

#[derive(Default, Clone, Copy)]
struct Frame {
    direction: u32, // 0 - 3
    current_frame: u32, // 0 - 2
}

#[derive(Default, Clone, Copy, PartialEq)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl<'a> Player<'a> {
    pub fn new(pos: Point, texture: Texture<'a>, sprite: Rect, moving: Movement, frame: Frame) -> Self {
	Self {
	    pos,
	    texture,
	    sprite,
	    moving,
	    frame,
	}
    }
    pub fn change_pos(&mut self) {
	match self.moving.direction {
	    Direction::Left => {
		self.pos = self.pos.offset(-self.moving.speed, 0);
	    },
	    Direction::Right => {
		self.pos = self.pos.offset(self.moving.speed, 0);
	    },
	    Direction::Up => {
		self.pos = self.pos.offset(0, -self.moving.speed);
	    },
	    Direction::Down => {
		self.pos = self.pos.offset(0, self.moving.speed);
	    }
	}
    }
    pub fn get_frame(&mut self) {
	match self.moving.direction {
	    Direction::Left => {
		self.frame.direction = 1 as u32;
	    },
	    Direction::Right => {
		self.frame.direction = 2 as u32;
	    },
	    Direction::Up => {
		self.frame.direction = 3 as u32;
	    },
	    Direction::Down => {
		self.frame.direction = 0 as u32;
	    },
	}
	if self.moving.previous_direction == self.moving.direction {
	    self.frame.current_frame = (self.frame.current_frame + 1) % 2;
	}
    }
    pub fn change_frame(&mut self) {
	self.sprite = Rect::new(0 + (32 * self.frame.current_frame) as i32 ,0 + (36 * self.frame.direction) as i32,26, 36);
    }
    pub fn update_player(player: &mut Player) {
	if player.moving.current {
	    player.change_pos();
	    player.get_frame();
	    player.change_frame();
	    player.moving.previous_direction = player.moving.direction;
	}
	else {
	    player.frame.current_frame = 0 as u32;
	}
    }
}

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
    let texture = texture_creator.load_texture("c:/Users/black/projects/rust/RealVampire/assets/reaper.png")?;
    let texture_1 = texture_creator.load_texture("c:/Users/black/projects/rust/RealVampire/assets/reaper.png")?;
    let texture_2 = texture_creator.load_texture("c:/Users/black/projects/rust/RealVampire/assets/bardo.png")?;
    let mut player_list = Vec::new();
    let movement_struct = Movement {
	direction: Direction::Down,
	current: false,
	speed: 5,
	previous_direction: Direction::Down,
    };
    let frame = Frame {
	direction: 0,
	current_frame: 0,
    };
    let mut player = Player::new(Point::new(50,50), texture, Rect::new(0,0,26,36), movement_struct, frame);
    let player_1 = Player::new(Point::new(50,50), texture_1, Rect::new(0,0,26,36), movement_struct, frame);
    let player_2 = Player::new(Point::new(75,75), texture_2, Rect::new(0,0,26,36), movement_struct, frame);
    player_list.push(player_1);
    player_list.push(player_2);
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
	for event in event_pump.poll_iter() {
	    match event {
		Event::Quit {..} |
		Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
		    break 'running;
		},
		Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    player.moving.direction = Direction::Left;
		    player.moving.current = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
		    player.moving.direction = Direction::Right;
		    player.moving.current = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    player.moving.direction = Direction::Up;
		    player.moving.current = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
		    player.moving.direction = Direction::Down;
		    player.moving.current = true;
                },
		Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
		Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
		Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
		Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
		    player.moving.current = false;
		}
		_ => {}
	    }
	}
	// game loop
	// update function
	
	Player::update_player(&mut player);
	
	// render function
	render(&mut canvas, Color::RGB(0, 0, 0), &player_list, &player)?;
	// time management
	std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }
    Ok(())
}

fn render(canvas: &mut WindowCanvas, color: Color, player_list: &Vec<Player>, local_player: &Player ) -> Result<(), String> {
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
    let screen_rect = Rect::from_center(screen_position, local_player.sprite.width() * 2, local_player.sprite.height() * 2);
    canvas.copy(&local_player.texture, local_player.sprite, screen_rect)?;
    for player in player_list {
	let screen_position = player.pos + Point::new(width as i32 / 2, height as i32 / 2 );
	let screen_rect = Rect::from_center(screen_position, player.sprite.width() * 2, player.sprite.height() * 2);
	canvas.copy(&player.texture, player.sprite, screen_rect)?;
    }
    Ok(())
}
