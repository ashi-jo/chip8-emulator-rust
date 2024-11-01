use std::env;
use chip8_core::*;
use sdl2::event::Event;


const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run /path/to/game");
        return;
    }

    // setup sdl (drawing window)
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("CHIP-8 EMULATOR", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    // for looping the game and closing the game
    let mut event_pump = sdl_context.event_pump().unwrap();

    'gameloop: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit{..} => {
                    break 'gameloop;
                },
                _ => ()
            }
        }
    }
}

// Specifications

// 1. A 64x32 monochrome display, drawn to via sprites that are always 8 pixels wide and between 1 and 16 pixels tall
// 2. Sixteen 8-bit general purpose registers, referred to as V0 thru VF. VF also doubles as the flag register for overflow operations
// 3. 16-bit program counter
// 4. Single 16-bit register used as a pointer for memory access, called the I Register
// 5. 4KB RAM
// 6. 16-bit stack used for calling and returning from subroutines
// 7. 16-key keyboard input
// 8. Two special registers which decrease each frame and trigger upon reaching zero:

//Delay timer: Used for time-based game events
//Sound timer: Used to trigger the audio beep