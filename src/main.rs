extern crate rand;
extern crate sdl2;

use std::process;
use std::env::args;
use std::time::{Instant, Duration};

mod cpu;
mod bus;
mod rom;

use cpu::Cpu;
use bus::Bus;
use rom::Rom;

use sdl2::rect::{Rect};
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

fn main() {
    let rom_file = args().nth(1).unwrap();

    let rom = Rom::new(&rom_file).unwrap();

    let bus = Bus::new(rom);

    let mut cpu = Cpu::new(bus);

    let mut now = Instant::now();
    let mut last_instruction = now.clone();
    let mut last_screen = now.clone();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window  = video_subsystem.window("CHIP-8 Emulator by Vitaly Shvetsov", 640, 320)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    let mut rect = Rect::new(10, 10, 10, 10);
    
    let black = sdl2::pixels::Color::RGB(0, 0, 0);
    let white = sdl2::pixels::Color::RGB(255, 255, 255);

    let mut events = sdl_context.event_pump().unwrap();

    loop {
        now = Instant::now();
        if now - last_instruction > Duration::from_millis(2) {
            cpu.run_next_instruction();

            last_instruction = now.clone();        
            
            cpu.decrease_timers(now.clone());
            
            if now - last_screen > Duration::from_millis(10) {
           
                let _ = renderer.set_draw_color(black);
                let _ = renderer.clear();


                for x in 0..64 {
                    for y in 0..32 {
                        if is_paint(x, y, &cpu.video) {
                            let x_pos = (x * 10) as i32;
                            let y_pos = (y * 10) as i32;
                            rect.set_y(y_pos);
                            rect.set_x(x_pos);
                            let _ = renderer.fill_rect(rect);
                            let _ = renderer.set_draw_color(white);
                        }
                    }
                }
                let _ = renderer.present();
                
                last_screen = now.clone();
            }

            for event in events.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                        process::exit(1);
                    },

                    Event::KeyDown { keycode: Some(Keycode::X), ..} => {
                        cpu.read_keys(0x0, true);
                    },

                    Event::KeyDown { keycode: Some(Keycode::Num1), ..} => {
                        cpu.read_keys(0x1, true);
                    },

                    Event::KeyDown { keycode: Some(Keycode::Num2), ..} => {
                        cpu.read_keys(0x2, true);
                    },

                    Event::KeyDown { keycode: Some(Keycode::Num3), ..} => {
                        cpu.read_keys(0x3, true);
                    },

                    Event::KeyDown { keycode: Some(Keycode::Q), ..} => {
                        cpu.read_keys(0x4, true);
                    },

                    Event::KeyDown { keycode: Some(Keycode::W), ..} => {
                        cpu.read_keys(0x5, true);
                    },

                    Event::KeyDown { keycode: Some(Keycode::E), ..} => {
                        cpu.read_keys(0x6, true);
                    },

                    Event::KeyDown { keycode: Some(Keycode::A), ..} => {
                        cpu.read_keys(0x7, true);
                    },

                    Event::KeyDown { keycode: Some(Keycode::S), ..} => {
                        cpu.read_keys(0x8, true);
                    },

                    Event::KeyDown { keycode: Some(Keycode::D), ..} => {
                        cpu.read_keys(0x9, true);
                    },

                    Event::KeyDown { keycode: Some(Keycode::Z), ..} => {
                        cpu.read_keys(0xa, true);
                    },

                    Event::KeyDown { keycode: Some(Keycode::C), ..} => {
                        cpu.read_keys(0xb, true);
                    },

                    Event::KeyDown { keycode: Some(Keycode::Num4), ..} => {
                        cpu.read_keys(0xc, true);
                    },

                    Event::KeyDown { keycode: Some(Keycode::R), ..} => {
                        cpu.read_keys(0xd, true);
                    },

                    Event::KeyDown { keycode: Some(Keycode::F), ..} => {
                        cpu.read_keys(0xe, true);
                    },

                    Event::KeyDown { keycode: Some(Keycode::V), ..} => {
                        cpu.read_keys(0xf, true);
                    },

                    Event::KeyUp { keycode: Some(Keycode::X), ..} => {
                        cpu.read_keys(0x0, false);
                    },

                    Event::KeyUp { keycode: Some(Keycode::Num1), ..} => {
                        cpu.read_keys(0x1, false);
                    },

                    Event::KeyUp { keycode: Some(Keycode::Num2), ..} => {
                        cpu.read_keys(0x2, false);
                    },

                    Event::KeyUp { keycode: Some(Keycode::Num3), ..} => {
                        cpu.read_keys(0x3, false);
                    },

                    Event::KeyUp { keycode: Some(Keycode::Q), ..} => {
                        cpu.read_keys(0x4, false);
                    },

                    Event::KeyUp { keycode: Some(Keycode::W), ..} => {
                        cpu.read_keys(0x5, false);
                    },

                    Event::KeyUp { keycode: Some(Keycode::E), ..} => {
                        cpu.read_keys(0x6, false);
                    },

                    Event::KeyUp { keycode: Some(Keycode::A), ..} => {
                        cpu.read_keys(0x7, false);
                    },

                    Event::KeyUp { keycode: Some(Keycode::S), ..} => {
                        cpu.read_keys(0x8, false);
                    },

                    Event::KeyUp { keycode: Some(Keycode::D), ..} => {
                        cpu.read_keys(0x9, false);
                    },

                    Event::KeyUp { keycode: Some(Keycode::Z), ..} => {
                        cpu.read_keys(0xa, false);
                    },

                    Event::KeyUp { keycode: Some(Keycode::C), ..} => {
                        cpu.read_keys(0xb, false);
                    },

                    Event::KeyUp { keycode: Some(Keycode::Num4), ..} => {
                        cpu.read_keys(0xc, false);
                    },

                    Event::KeyUp { keycode: Some(Keycode::R), ..} => {
                        cpu.read_keys(0xd, false);
                    },

                    Event::KeyUp { keycode: Some(Keycode::F), ..} => {
                        cpu.read_keys(0xe, false);
                    },

                    Event::KeyUp { keycode: Some(Keycode::V), ..} => {
                        cpu.read_keys(0xf, false);
                    },

                    _ => {}
                }
            }
        }
    }
}

fn is_paint(x: usize, y: usize, vid_buffer: &[[u8; 64]; 32]) -> bool {
    vid_buffer[y][x] == 1
}

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}
