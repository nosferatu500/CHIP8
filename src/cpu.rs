use rand::Rng;
use rand::thread_rng;

use bus::Bus;

use std::time::{Instant, Duration};

pub struct Cpu {
  bus: Bus,

  pc: u16,
  sp: u8,

  stack: [u16; 16],

  i: usize,

  v: [u8; 16],

  pub video: [[u8; 64]; 32],

  key: [bool; 16],

  delay_timer: u8,
  
  sound_timer: u8,

  delay_duration: Instant,
}

impl Cpu {
  pub fn new(bus: Bus) -> Cpu {
    Cpu {
      bus,

      pc: 0x200,
      sp: 0,

      stack: [0; 16],

      i: 0,

      v: [0; 16],

      video: [[0; 64]; 32],

      key: [false; 16],

      delay_timer: 0,
      
      sound_timer: 0,

      delay_duration: Instant::now(),
    }
  }

  pub fn run_next_instruction(&mut self) {
    let lhs = self.bus.load(self.pc) as u16;
    let rhs = self.bus.load(self.pc + 1) as u16;

    let instruction = ((lhs << 8) | rhs) as u16;


    self.pc = self.pc.wrapping_add(2);

    self.decode(instruction); 
    
  }

  fn set_v(&mut self, addr: u8, value: u8) {
    self.v[addr as usize] = value;
  }

  fn get_v(&self, addr: u8) -> u8 {
    self.v[addr as usize]
  }

  pub fn decrease_timers(&mut self, now: Instant) {
      if now - self.delay_duration > Duration::from_millis(16) {
          if self.delay_timer > 0 {
              self.delay_timer -= 1;
          }
          if self.sound_timer > 0 {
              self.sound_timer -= 1;
          }
          self.delay_duration = Instant::now();
      }
  }

  fn decode(&mut self, instruction: u16) {
    let opcode = instruction >> 12;

    let nnn = instruction & 0x0fff;
    
    let nn = (instruction & 0x00ff) as u8;

    let n = (instruction & 0x000f) as u8;

    let x = ((instruction & 0x0f00) >> 8) as u8;

    let y = ((instruction & 0x00f0) >> 4) as u8;

    //println!("{:#06x}", instruction);

    match opcode {
      0x0 => {
        match nn {
          0xe0 => {
            self.video = [[0; 64]; 32];
          }
          0xee => {
            self.pc = self.stack[self.sp as usize];
            self.sp = self.sp.wrapping_sub(1);
          }
          _ => panic!("Unknown instruction {:#06x}", instruction),
        }
      }
      0x1 => {
        self.pc = nnn;
      }
      0x2 => {
        self.sp = self.sp.wrapping_add(1);
        self.stack[self.sp as usize] = self.pc;
        self.pc = nnn;
      }
      0x3 => {
        let vx = self.get_v(x);
        if vx == nn {
          self.pc = self.pc.wrapping_add(2);
        }
      }
      0x4 => {
        let vx = self.get_v(x);
        if vx != nn {
          self.pc = self.pc.wrapping_add(2);
        }
      }
      0x5 => {
        let vx = self.get_v(x);
        let vy = self.get_v(y);
        if vx == vy {
          self.pc = self.pc.wrapping_add(2);
        }
      }
      0x6 => {
        self.set_v(x, nn);
      }
      0x7 => {
        let old_v = self.get_v(x);
        self.set_v(x, old_v.wrapping_add(nn));
      }
      0x8 => {
        match n {
          0x0 => {
            let vy = self.get_v(y);
            self.set_v(x, vy);
          }
          0x1 => {
            let vx = self.get_v(x);
            let vy = self.get_v(y);
            self.set_v(x, vx | vy);
          }
          0x2 => {
            let vx = self.get_v(x);
            let vy = self.get_v(y);
            self.set_v(x, vx & vy);
          }
          0x3 => {
            let vx = self.get_v(x);
            let vy = self.get_v(y);
            self.set_v(x, vx ^ vy);
          } 
          0x4 => {
            let vx = self.get_v(x);
            let vy = self.get_v(y);

            let old_vx = vx;

            self.set_v(x, vx.wrapping_add(vy));

            let new_vx = self.get_v(x);

            if new_vx < old_vx {
              self.set_v(0xf, 1);
            } else {
              self.set_v(0xf, 0);
            }
          }

          0x5 => {
            let vx = self.get_v(x);
            let vy = self.get_v(y);

            let old_vx = vx;

            self.set_v(x, vx.wrapping_sub(vy));

            let new_vx = self.get_v(x);

            if new_vx > old_vx {
              self.set_v(0xf, 1);
            } else {
              self.set_v(0xf, 0);
            }
          }

          0x6 => {
            let vy = self.get_v(y);

            self.set_v(y, vy >> 1);
            self.set_v(x, vy >> 1);

            self.set_v(0xf, vy & 1);
          }

          0x7 => {
            let vx = self.get_v(x);
            let vy = self.get_v(y);

            let old_vy = vy;

            self.set_v(x, vy.wrapping_sub(vx));

            let new_vy = self.get_v(y);

            if new_vy > old_vy {
              self.set_v(0xf, 1);
            } else {
              self.set_v(0xf, 0);
            }
          }

          0xe => {
            let vy = self.get_v(y);

            self.set_v(y, vy << 1);
            self.set_v(x, vy << 1);

            self.set_v(0xf, vy >> 7);
          }
          _ => panic!("Unknown instruction {:#06x}", instruction),
        }
      }
      0x9 => {
        let vx = self.get_v(x);
        let vy = self.get_v(y);
        if vx != vy {
          self.pc = self.pc.wrapping_add(2);
        }
      }
      0xa => {
        self.i = nnn as usize;
      }
      0xb => {
        let value = self.get_v(0);
        self.pc = nnn.wrapping_add(value as u16);
      }
      0xc => {
        let rand = thread_rng().gen::<u32>();
        self.set_v(x, (rand & nn as u32) as u8);
      }
      0xd => {
        self.draw(x, y, n);
      }
      0xe => {
        match nn {
          0x9e => {
            let key = self.key[self.get_v(x) as usize];
            
            if key {
              self.pc = self.pc.wrapping_add(2);
            }
          }
          0xa1 => {
            let key = self.key[self.get_v(x) as usize];
            
            if !key {
              self.pc = self.pc.wrapping_add(2);
            }
          }
          _ => panic!("Unknown instruction {:#06x}", instruction),
        }
      }
      0xf => {
        match nn {
          0x07 => {
            let value = self.delay_timer;
            self.set_v(x, value);
          }
          0x0a => {
            let mut pressed = false;

            for index in 0x0..0xf {
              if self.key[index] {
                self.set_v(x, index as u8);
                pressed = true;
              } 
            }

            if !pressed {
              // Blocking Operation. All instruction halted until next key event.
              self.pc = self.pc.wrapping_sub(2);
            }
          }
          0x15 => {
            let value = self.get_v(x);
            self.delay_timer = value;
          }
          0x18 => {
            let value = self.get_v(x);
            self.sound_timer = value;
          }
          0x1e => {
            let vx = self.get_v(x);
            self.i = self.i.wrapping_add(vx as usize);
          }
          0x29 => {
            let value = self.get_v(x);
            self.i = value as usize * 5; // Font 4x5.
          }
          0x33 => {
            let value = self.get_v(x);
            self.bus.store(self.i as u16, value / 100);
            self.bus.store((self.i + 1) as u16, (value % 100) / 10);
            self.bus.store((self.i + 2) as u16, value % 10);
            
          }
          0x55 => {
            for index in 0..x {
              let value = self.get_v(index);
              self.bus.store((self.i as u8 + index) as u16, value);
            }
          }
          0x65 => {
            for index in 0..x {
              let value = self.bus.load((self.i as u8 + index) as u16);
              self.set_v(index, value);
            }
          }
         _ => panic!("Unknown instruction {:#06x}", instruction),
        }
      }
      _ => panic!("Unknown instruction {:#06x}", instruction),
    }
  }

  fn draw(&mut self, x: u8, y: u8, n: u8) {
    let col = self.get_v(x) % 64;
    let row = self.get_v(y) % 32;

    self.set_v(0xf, 0);
    
    for offset in 0..n {
      let pixel = self.bus.rom.load(self.i.wrapping_add(offset as usize) as u16);
      for coll_offset in 0..8 {
        if (pixel & 0x80 >> coll_offset) > 0 {
          if self.video[((row + offset) % 32) as usize][((col + coll_offset) % 64) as usize] == 1 {
            self.set_v(0xf, 1);
          }
          self.video[((row + offset) % 32) as usize][((col + coll_offset) % 64) as usize] ^= 1;
        }
      }
    }
  }

  pub fn read_keys(&mut self, key_code: usize, status: bool) {
    self.key[key_code] = status;
  }
}
