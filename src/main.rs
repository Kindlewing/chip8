use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::sys::SDL_Point;
use std::time::Duration;

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
    display: [SDL_Point; 64 * 32],
    pc: u8,
    i: [u16; 1],
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    reg: [u8; 16],
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            memory: [0; 4096],
            display: [SDL_Point { x: 0, y: 0 }; 64 * 32],
            pc: 0,
            i: [0; 1],
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            reg: [0; 16],
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let scale: u32 = 10;

    let window = video_subsystem
        .window("Chip8", 64 * scale, 32 * scale)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let chip8: Chip8 = Chip8::new();
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
        // The rest of the game loop goes here...

        canvas.present();
    }
}
