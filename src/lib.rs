use rand::random;

const DEFAULT_STACK_CAPACITY: usize = 64;
const INSTRUCTION_SIZE: u16 = 2;

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
pub enum Register {
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

impl From<u8> for Register {
    fn from(i: u8) -> Self {
        use Register::*;
        match i {
            0x0 => V0,
            0x1 => V1,
            0x2 => V2,
            0x3 => V3,
            0x4 => V4,
            0x5 => V5,
            0x6 => V6,
            0x7 => V7,
            0x8 => V8,
            0x9 => V9,
            0xA => VA,
            0xB => VB,
            0xC => VC,
            0xD => VD,
            0xE => VE,
            0xF => VF,
            _ => unreachable!(),
        }
    }
}

impl From<u16> for Register {
    fn from(i: u16) -> Self {
        (i as u8).into()
    }
}

#[derive(Debug)]
pub struct Emulator {
    memory: [u8; 4096],
    registers: [u8; 16],
    address_register: u16,
    stack: Vec<u16>,
    instruction_pointer: u16,
    paused: bool,
}

impl Emulator {
    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn unpause(&mut self) {
        self.paused = false;
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn load(&mut self, rom: &[u8]) -> std::io::Result<()> {
        for (i, b) in rom.iter().enumerate() {
            self.memory[i + 0x200] = *b;
        }

        Ok(())
    }

    pub fn load_file(&mut self, path: &str) -> std::io::Result<()> {
        let rom = std::fs::read(path)?;
        self.load(&rom)
    }

    pub fn step(&mut self) -> std::io::Result<()> {
        if self.paused {
            self.paused = false;
            self.clock()?;
            self.paused = true;
        }
        Ok(())
    }

    pub fn clock(&mut self) -> std::io::Result<()> {
        if self.paused {
            return Ok(());
        }
        let instruction = self.read_instruction()?;
        self.instruction_pointer += INSTRUCTION_SIZE;

        self.execute(instruction)?;

        Ok(())
    }

    fn execute(&mut self, instruction: u16) -> std::io::Result<()> {
        match instruction {
            0x00EE => {
                let address = self.stack.pop().expect("stack was empty, you lark!");
                self.instruction_pointer = address;
            }
            0x1000..=0x1FFF => {
                let address = instruction & 0x0FFF;
                self.instruction_pointer = address;
            }
            0x2000..=0x2FFF => {
                let address = instruction & 0x0FFF;
                self.stack.push(self.instruction_pointer);
                self.instruction_pointer = address;
            }
            0x3000..=0x3FFF => {
                let whatever = instruction & 0x0FFF;
                let register = whatever >> 8;
                let nn = (whatever ^ (register << 8)) as u8;
                if nn == self.read_register(register.into()) {
                    self.instruction_pointer += INSTRUCTION_SIZE;
                }
            }
            0x6000..=0x6FFF => {
                let whatever = instruction & 0x0FFF;
                let register = whatever >> 8;
                let nn = (whatever ^ (register << 8)) as u8;

                self.write_register(register.into(), nn);
            }
            0x7000..=0x7FFF => {
                let whatever = instruction & 0x0FFF;
                let register = whatever >> 8;
                let nn = (whatever ^ (register << 8)) as u8;
                let register = register.into();
                let old = self.read_register(register);
                self.write_register(register, old + nn);
            }
            0xA000..=0xAFFF => {
                let address = instruction & 0x0FFF;
                self.write_address_register(address);
            }
            0xC000..=0xCFFF => {
                let whatever = instruction & 0x0FFF;
                let register = whatever >> 8;
                let nn = (whatever ^ (register << 8)) as u8;
                let r: u8 = random();
                self.write_register(register.into(), nn & r);
            }
            0xD000..=0xDFFF => {
            }
            _ => {
                println!("{:#06X}", instruction);
                unimplemented!();
            }
        }

        Ok(())
    }

    pub fn instruction_pointer(&self) -> u16 {
        self.instruction_pointer
    }

    fn read_instruction(&mut self) -> std::io::Result<u16> {
        use byteorder::{BigEndian, ReadBytesExt};
        let instruction = [
            self.read_memory(self.instruction_pointer),
            self.read_memory(self.instruction_pointer + 1),
        ];
        let mut cursor = std::io::Cursor::new(instruction);
        cursor.read_u16::<BigEndian>()
    }

    pub fn read_memory(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn read_address_register(&self) -> u16 {
        self.address_register
    }

    pub fn write_address_register(&mut self, value: u16) {
        self.address_register = value;
    }

    pub fn read_register(&self, register: Register) -> u8 {
        self.registers[register as usize]
    }

    pub fn write_register(&mut self, register: Register, value: u8) {
        self.registers[register as usize] = value;
    }
}

impl Default for Emulator {
    fn default() -> Self {
        Emulator {
            memory: [0; 4096],
            registers: [0; 16],
            address_register: 0,
            stack: Vec::with_capacity(DEFAULT_STACK_CAPACITY),
            instruction_pointer: 0x200,
            paused: false,
        }
    }
}
