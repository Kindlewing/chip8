use sdl2::sys::SDL_Point;

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
    V10,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
}

struct chip_8 {
    memory: [u8; 4096],
    display: [SDL_Point; 64 * 32],
}

fn main() {}
