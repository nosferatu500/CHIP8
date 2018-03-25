use rom::Rom;

mod map {
    pub struct Range(u16, u16);

    impl Range {
        pub fn contains(self, addr: u16) -> Option<u16> {
            let Range(start, end) = self;

            if addr >= start && addr < start + (end - start + 0x1) {
                Some(addr - start)
            } else {
                None
            }
        }
    }

    pub const ROM: Range = Range(0x000, 0xFFF);
}

pub struct Bus {
  pub rom: Rom,
}

impl Bus {
  pub fn new(rom: Rom) -> Bus {
    Bus {
      rom,
    }
  }

  pub fn load(&self, addr: u16) -> u8 {
      if let Some(offset) = map::ROM.contains(addr) {
          return self.rom.load(offset);
      }
      
      panic!("Unhandled load 8bit address {:#08x}", addr);
  }

  pub fn store(&mut self, addr: u16, value: u8) {
      if let Some(offset) = map::ROM.contains(addr) {
          return self.rom.store(offset, value);
      }

      panic!("Unhandled store 8bit address {:#x}", addr);
  }
}
