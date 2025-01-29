use sdl2::{rect::{Point, Rect},render::{Texture, }};

pub struct Player<'a> {
    pub pos: Point,
    pub sprite: Sprite<'a>,
    pub velocity: Velocity,
    pub frame: Frame,
}

#[derive(Default, Clone, Copy)]
pub struct Velocity {
    pub direction: Direction,
    pub current: bool,
    pub speed: i32,
    pub previous_direction: Direction, 
}

pub struct Sprite<'a> {
    pub size: Rect,
    pub texture: Texture<'a>,
}

#[derive(Default, Clone, Copy)]
pub struct Frame {
    pub direction: u32, // 0 - 3
    pub current_frame: u32, // 0 - 2
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl<'a> Player<'a> {
    pub fn new(pos: Point, sprite: Sprite<'a>, velocity: Velocity, frame: Frame) -> Self {
	Self {
	    pos,
	    sprite,
	    velocity,
	    frame,
	}
    }
    pub fn change_pos(&mut self) {
	match self.velocity.direction {
	    Direction::Left => {
		self.pos = self.pos.offset(-self.velocity.speed, 0);
	    },
	    Direction::Right => {
		self.pos = self.pos.offset(self.velocity.speed, 0);
	    },
	    Direction::Up => {
		self.pos = self.pos.offset(0, -self.velocity.speed);
	    },
	    Direction::Down => {
		self.pos = self.pos.offset(0, self.velocity.speed);
	    }
	}
    }
    pub fn get_frame(&mut self) {
	match self.velocity.direction {
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
	if self.velocity.previous_direction == self.velocity.direction {
	    self.frame.current_frame = (self.frame.current_frame + 1) % 2;
	}
    }
    pub fn change_frame(&mut self) {
	self.sprite.size = Rect::new(0 + (32 * self.frame.current_frame) as i32 ,0 + (36 * self.frame.direction) as i32,26, 36);
    }
    pub fn update_player(player: &mut Player) {
	if player.velocity.current {
	    player.change_pos();
	    player.get_frame();
	    player.change_frame();
	    player.velocity.previous_direction = player.velocity.direction;
	}
	else {
	    player.frame.current_frame = 0 as u32;
	}
    }
}

