use macroquad::prelude::*;

pub struct Chip8 {
    pub ram: [u8; 0x1000],
    pub reg_v: [u8; 0x10],
    pub stack: [u8; 0x10],
    pub reg_i: u16,
    pub pc: u16,
    pub sp: u8,
}

impl Chip8 {
    /// Create a new instance of a Chip8.
    pub fn new() -> Chip8 {
        let ram = [0; 0x1000];
        let reg_v = [0; 0x10];
        let stack = [0; 0x10];
        let reg_i = 0;
        let pc = 0x200;
        let sp = 0x0;

        Chip8 { ram, reg_v, stack, reg_i, pc, sp, }
    }

    /// Load instructions from a rom file to the ram.
    pub async fn load_rom(&mut self, path: &str) -> &mut Chip8 {
        let mut i = 0x200;

        let rom = match load_file(path).await {
            Ok(o) => o,
            Err(e) => panic!("{}", e),
        };

        for x in &rom {
            self.ram[i] = *x;
            i += 1;
        }

        self
    }

    /// Print the contents of the ram from 'from' to 'to - 1'.
    pub fn dump_data(&self, from: usize, to: usize) {
        println!(" === RAM");
        for i in from..to {
            print!("{:0>2x}", self.ram[i]);
            if (i - from) % 32 == 31 {
                print!("\n");
            } else if (i - from) % 2 == 1 {
                print!(" ");
            }
        }
        println!("\n\n === V");
        for i in 0..16 {
            print!("{:x}={:0>2x} ", i, self.reg_v[i]);
        }
        println!("\n\n === STACK");
        for i in 0..16 {
            print!("{:x}={:0>2x} ", i, self.stack[i]);
        }
        println!("\n\n === OTHER");
        print!("I={:x} PC={:x} SP={:x}", self.reg_i, self.pc, self.sp);
        println!("\n");
    }

    pub fn fetch(&self) -> u16 {
        0x0000 | ((self.ram[self.pc as usize] as u16) << 8) | 
            self.ram[self.pc as usize + 1] as u16
    }

    pub fn inc_pc(&mut self) -> &mut Chip8 {
        self.pc += 2;
        if self.pc > 0x1000 {
            panic!("Program counter overflowed!");
        }
        self
    }

    pub fn cls_0(&mut self) -> &mut Chip8 {
        self
    }

    pub fn ret_0(&mut self) -> &mut Chip8 {
        self
    }

    pub fn jp_1(&mut self, instruction: u16) -> &mut Chip8 {
        let addr = instruction & 0x0FFF;
        self
    }

    pub fn call_2(&mut self, instruction: u16) -> &mut Chip8 {
        let addr = instruction & 0x0FFF;
        self
    }

    pub fn se_3(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        let byte = instruction & 0x00FF;
        self
    }

    pub fn sne_4(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        let byte = instruction & 0x00FF;
        self
    }

    pub fn se_5(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        let y = (instruction & 0x00F0) >> 4;
        self
    }

    pub fn ld_6(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        let byte = instruction & 0x00FF;
        self
    }

    pub fn add_7(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        let byte = instruction & 0x00FF;
        self
    }

    pub fn ld_8(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        let y = (instruction & 0x00F0) >> 4;
        self
    }

    pub fn or_8(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        let y = (instruction & 0x00F0) >> 4;
        self
    }

    pub fn and_8(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        let y = (instruction & 0x00F0) >> 4;
        self
    }

    pub fn xor_8(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        let y = (instruction & 0x00F0) >> 4;
        self
    }

    pub fn add_8(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        let y = (instruction & 0x00F0) >> 4;
        self
    }

    pub fn sub_8(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        let y = (instruction & 0x00F0) >> 4;
        self
    }

    pub fn shr_8(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        let y = (instruction & 0x00F0) >> 4;
        self
    }

    pub fn subn_8(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        let y = (instruction & 0x00F0) >> 4;
        self
    }

    pub fn shl_8(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        let y = (instruction & 0x00F0) >> 4;
        self
    }

    pub fn sne_9(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        let y = (instruction & 0x00F0) >> 4;
        self
    }

    pub fn ld_a(&mut self, instruction: u16) -> &mut Chip8 {
        let addr = instruction & 0x0FFF;
        self
    }

    pub fn jp_b(&mut self, instruction: u16) -> &mut Chip8 {
        let addr = instruction & 0x0FFF;
        self
    }

    pub fn rnd_c(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        let byte = instruction & 0x00FF;
        self
    }

    pub fn drw_d(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        let y = (instruction & 0x00F0) >> 4;
        let n = instruction & 0x000F;
        self
    }

    pub fn skp_e(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        self
    }

    pub fn sknp_e(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        self
    }

    pub fn ld_dt(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        self
    }

    pub fn ld_k(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        self
    }

    pub fn ld_i(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        self
    }

    pub fn ld_f(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        self
    }

    pub fn ld_b(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        self
    }

    pub fn ld_arr_x(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        self
    }

    pub fn ld_x_arr(&mut self, instruction: u16) -> &mut Chip8 {
        let x = (instruction & 0x0F00) >> 8;
        self
    }


    pub fn ignore(&mut self) -> &mut Chip8 {
        // println!("ignore @ {:#x}", self.pc);
        self
    }

    pub fn unknown(&mut self) -> &mut Chip8 {
        // println!("unknown @ {:#x}", self.pc);
        self
    }
}
