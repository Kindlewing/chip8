use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::sys::SDL_Point;

const CHIP8_WIDTH: u32 = 64;
const CHIP8_HEIGHT: u32 = 32;

enum Register {
	V0,
	V1,
	V2,
	V3,
	V4,
	V5,
	V6,
	V7,
	V8,
	V9,
	VA,
	VB,
	VC,
	VD,
	VE,
	VF,
}

struct Chip8 {
	memory: [u8; 4096],
	display: [u8; 64 * 32],
	pc: u16,
	i: u16,
	stack: Vec<u16>,
	delay_timer: u8,
	sound_timer: u8,
	reg: [u8; 16],
}

impl Chip8 {
	pub fn new() -> Self {
		Chip8 {
			memory: [0; 4096],
			display: [0; CHIP8_WIDTH as usize * CHIP8_HEIGHT as usize],
			pc: 0x200,
			i: 0,
			stack: Vec::new(),
			delay_timer: 0,
			sound_timer: 0,
			reg: [0; 16],
		}
	}

	pub fn fetch(&mut self) -> u16 {
		let upper = self.memory[self.pc as usize];
		let lower = self.memory[(self.pc as usize) + 1];
		self.pc += 2;
		(upper as u16) << 8 | lower as u16
	}
}

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let scale: u32 = 10;

	let window = video_subsystem
		.window("Chip8", CHIP8_WIDTH * scale, CHIP8_HEIGHT * scale)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();

	canvas.clear();
	canvas.present();
	let mut event_pump = sdl_context.event_pump().unwrap();

	let mut chip8: Chip8 = Chip8::new();
	'running: loop {
		canvas.clear();
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. }
				| Event::KeyDown {
					keycode: Some(Keycode::Escape),
					..
				} => break 'running,
				_ => {}
			}
		}
		let opcode = chip8.fetch();
		let instr = (opcode & 0xF000) >> 12 as u8;
		let x = (opcode & 0x0F00) >> 8 as u8;
		let y = (opcode & 0x00F0) >> 4 as u8;
		let n = (opcode & 0x000F) as u8;

		let nn = (opcode & 0x00FF) as u8;
		let nnn = (opcode & 0x0FFF) as usize;

		match (instr, x, y, n) {
			// clear screen
			(0x00, 0x00, 0x0E, 0x00) => {
				println!("clear screen");
			}
			// jump
			(0x01, _, _, _) => {
				chip8.pc = nnn as u16;
			}

			// set register VX
			(0x06, _, _, _) => {
				chip8.reg[x as usize] = nn;
			}
			// add
			(0x07, _, _, _) => {
				chip8.reg[x as usize] += nn;
			}

			// set I
			(0x0A, _, _, _) => {
				chip8.i = nnn as u16;
			}

			// display
			(0x0D, _, _, _) => {
				let x_pos = chip8.reg[x as usize] as u16 % 64;
				let y_pos = chip8.reg[y as usize] as u16 % 32;
				chip8.reg[0xF] = 0;

				for row in 0..n {
					let sprite = chip8.memory[chip8.i as usize + row as usize];
					for bit in 0..8 {
						if (x_pos as u32 + bit as u32) < CHIP8_HEIGHT
							&& (y_pos as u32 + row as u32) < CHIP8_WIDTH
						{
							if bit
								& chip8.display[x_pos as usize
									+ y_pos as usize * CHIP8_WIDTH as usize]
								== 1
							{
								chip8.display[x_pos as usize
									+ y_pos as usize * CHIP8_WIDTH as usize] = 0;
							}
						}
					}
				}
			}
			(_, _, _, _) => println!("Unrecognized opcode"),
		}

		canvas.present();
	}
}
