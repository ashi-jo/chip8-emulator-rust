use rand::random;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const NUM_REGS: usize = 16;
const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const NUM_SIZE: usize = 16;

const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Emu {
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; NUM_REGS],
    i_reg: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    dt: u8,
    st: u8,
}

// -- Unchanged code omitted --

const START_ADDR: u16 = 0x200;

// -- Unchanged code omitted --

impl Emu {
    pub fn new() -> Self {
        let mut new_emu = Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        }

        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET)

        new_emu
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn tick(&mut self) {
        // Fetch
        let op = self.fetch();
        // Decode and Execute
        self.execute(op);
    }

    fn execute(&mut self, op: u16) {
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            let dig2_usize = digit2 as usize;
            let dig3_usize = digit3 as usize;

            // LOAD V0 - VX
            (0xF, _, 6, 5) => {
                let i = self.i_reg as usize;
                for idx in 0..=dig2_usize {
                    self.v_reg[idx] = self.ram[dig2_usize + idx];
                }
            },
            // STORE V0 - VX
            (0xF, _, 5, 5) => {
                let i = self.i_reg as usize;
                for idx in 0..=dig2_usize {
                    self.ram[dig2_usize + idx] = self.v_reg[idx];
                }
            },
            // convert from HEX to BCD for display purposes
            (0xF, _, _, 3) => {
                let vx = self.v_reg[dig2_usize] as f32;
                
                let hundereds = (vx / 100.0).floor() as u8;
                let tens = ((vx / 10.0) % 10.0).floor() as u8;
                let ones = (vx % 10) as u8;

                self.ram[self.i_reg as usize] = hundereds;
                self.ram[(self.i_reg + 1) as usize] = tens;
                self.ram[(self.i_reg + 2) as usize] = ones;
            },
            // I = FONT
            (0xF, _, 2, 9) => {
                let c = self.v_reg[dig2_usize] as u16;
                self.i_reg = c * 5;
            },
            // I += VX
            (0xF, _, 1, 0xE) => {
                let vx = self.v_reg[dig2_usize] as u16;
                self.i_reg = self.i_reg.wrapping_add(vx); 
            }
            // ST = VX
            (0xF, _, 1, 8) => {
                self.st = self.v_reg[dig2_usize];
            },
            // DT = VX
            (0xF, _, 1, 5) => {
                self.dt = self.v_reg[dig2_usize];
            },
            // wait key 
            (0xF, _, 0, 0xA) => {
                let mut pressed = false;
                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_reg[dig2_usize] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    // Redo opcode
                    self.pc -= 2;
                }
            },
            // VX = DT
            (0xF, _, 0, 7) => {
                self.v_reg[dig2_usize] = self.dt;
            },
            //skip if key not press 
            (0xA, _, 0xA, 1) = {
                let vx = self.v_reg[dig2_usize];
                let key = self.keys[vx as usize];
                if !key {
                    self.pc += 2;
                }
            },
            //skip key press 
            (0xE, _, 9, 0xE) = {
                let vx = self.v_reg[dig2_usize];
                let key = self.keys[vx as usize];
                if key {
                    self.pc += 2;
                }
            },
            // Draw
            (0xD, _, _, _) => {
                let x_coord = self.v_reg[dig2_usize] as u16;
                let y_coord = self.v_reg[dig3_usize] as u16;
                // last digit tells us how high our sprite (drawing is going to be)
                let num_rows = digit4;

                // keep track if pixels were flipped
                let mut flipped = false;

                for y_line in 0..num_rows {
                    // we will start to draw sprites at the memory loc stored in i reg
                    let addr = self.i_reg + y_line as u16;
                    let pixels = self.ram[addr as usize];

                    // iterate over each column or each "pixel" (1bit)
                    for x_line in 0..8 {
                        // Fetch current pixel's bit, only flip if 1 
                        // we shift the 1 in "1000 0000" to right and use it to check if x_line bit is 1 currently
                        if (pixels & (0b1000_0000 >> x_line)) !=0 {
                            // apply modulo to wrap around screen
                            let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;

                            // Get index for 1D screen array
                            let idx = x + SCREEN_WIDTH * y;
                            flipped |= self.screen[idx];
                            self.screen[idx] ^= true; 
                        }
                    }

                }

                // populate vf
                if flipped {
                    self.v_reg[0xF] = 1;
                } else {
                    self.v_reg[0xF] = 0;
                }
            },
            // VX = rand() & NN
            (0xC, _, _, _) => {
                let nn = (op & 0xFF) as u8;
                let rng: u8 = random();
                self.v_reg[dig2_usize] = rng & nn;
            },
            // JMP V0 + NNN
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = (self.v_reg[0] as u16) + nnn;
            },
            // I = NNN
            (0xA, _, _, _) => {
                let nnn = op & 0xFFF;
                self.i_reg = nnn;
            },
            // SKIP VX != VY
            (9, _, _, 0) => {
                self.v_reg[dig2_usize] != self.v_reg[dig3_usize] {
                    self.pc += 2;
                }
            },
            // VX <<= 1
            (8, _, _, 0xE) => {
                let msb = (self.v_reg[dig2_usize] >> 7) & 1;
                self.v_reg[dig2_usize] <<= 1;
                self.v_reg[0xF] = msb;
            },
            // VX = VY - VX
            (8, _, _, 7) => {
                let (new_vx, borrow) = self.v_reg[dig3_usize].owerflowing_sub(self.v_reg[dig2_usize])
                let new_vf = if borrow {0} else {1};
                self.v_reg[dig2_usize] = new_vx;
                self.v_reg[0xF] = new_vf;
            },
            // VX >>= 1
            (8, _, _, 6) => {
                let lsb = self.v_reg[dig2_usize] & 1;
                self.v_reg[dig2_usize] >>= 1;
                self.v_reg[0xF] = lsb;
            },
            // VX -= VY
            (8, _, _, 5) => {
                let (new_vx, borrow) = self.v_reg[dig2_usize].owerflowing_sub(self.v_reg[dig3_usize])
                let new_vf = if borrow {0} else {1};
                self.v_reg[dig2_usize] = new_vx;
                self.v_reg[0xF] = new_vf;
            },
            // VX += VY
            (8, _, _, 4) => {
                let (new_vx, carry) = self.v_reg[dig2_usize].owerflowing_add(self.v_reg[dig3_usize])
                let new_vf = if carry {1} else {0};
                self.v_reg[dig2_usize] = new_vx;
                self.v_reg[0xF] = new_vf;
            },
            // VX ^= VY
            (8, _, _, 1) => {
                self.v_reg[dig2_usize] ^= self.v_reg[dig3_usize];
            },
            // VX &= VY
            (8, _, _, 1) => {
                self.v_reg[dig2_usize] &= self.v_reg[dig3_usize];
            },
            // VX |= VY
            (8, _, _, 1) => {
                self.v_reg[dig2_usize] |= self.v_reg[dig3_usize];
            },
            // VX = VY from reg
            (8, _, _, _) => {
                self.v_reg[dig2_usize] = self.v_reg[dig3_usize];
            },
            //  VX += NN
            (7, _, _, _) => {
                let nn = (op & 0xFF) as u8;
                self.v_reg[dig2_usize].wrapping_add(nn);
            },
            // set nn VX = NN
            (6, _, _, _) => {
                let nn = (op & 0xFF) as u8;
                self.v_reg[dig2_usize] = nn;
            },
            // SKIP VX == VY
            (5, _, _, 0) => {
                if self.v_reg[dig2_usize] == self.v_reg[dig3_usize] {
                    self.pc += 2;
                }
            },
            // Skip VX != NN
            (4, _, _, _) => {
                let nn = (op & 0xFF) as u8;
                if self.v_reg[dig2_usize] != nn {
                    self.pc +=2;
                }
            },
            // Skip VX == NN
            (3, _, _, _) => {
                let nn = (op & 0xFF) as u8;
                if self.v_reg[dig2_usize] == nn {
                    self.pc +=2;
                }
            },
            // 2NNN call subroutine
            (2, _, _, _) => {
                let nnn = op & 0x0FFF;
                self.push(self.pc)
                self.pc = nnn;
            },
            // Jump to NNN
            (1, _, _, _) => {
                let nnn = op & 0x0FFF;
                self.pc = nnn;
            },
            // RET
            (0, 0, 0xE, 0xE) => {
                let ret_addr = self.pop();
                self.pc = ret_addr;
            },
            // CLS
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            },
            // NOP
            (0, 0, 0, 0) => return, 
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", op)
        }

    }
    
    fn fetch(&mut self) -> u16 {
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
        let op = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        op
    }

    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // BEEP
                println("Must BEEP");
            }
            self.st -= 1;
        }
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

}
