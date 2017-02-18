use mix_operations::*;
use mix_operations::Operation::*;

const max_byte_value: u32 = 1 << 30;
const mem_size: u16 = 4000;

struct MixMachine {
    register_A: u32,    // N.B.: Unlike true MIX, we don't allow both -0 and +0
    register_X: u32,
    register_I1: u16,
    register_I2: u16,
    register_I3: u16,
    register_I4: u16,
    register_I5: u16,
    register_I6: u16,
    register_J: u16,
    program_counter: u16,   // Not strictly specified in MIX, but needed!
    memory: [u32; mem_size as usize]
}

struct MixMachineErr {
    message: String,
}

pub enum Register { RegA, RegX, RegI1, RegI2, RegI3, RegI4, RegI5, RegI6, RegJ }

impl MixMachine {
    fn new() -> MixMachine {
        MixMachine{
            register_A: 0u32, register_X: 0u32,
            register_I1: 0u16, register_I2: 0u16,
            register_I3: 0u16, register_I4: 0u16,
            register_I5: 0u16, register_I6: 0u16,
            program_counter: 0u16,
            register_J: 0u16, memory: [0; mem_size as usize]
        }
    }

    fn poke_memory(&mut self, address: u16, value: u32) -> Result<(), MixMachineErr> {
        if (address >= mem_size) {
            Err(MixMachineErr{message: format!("Attempt to access invalid memory address {}.", address)})
        } else if (value > max_byte_value) {
            Err(MixMachineErr{message: format!("Attempt to write invalid value {}.", value)})
        } else {
            self.memory[address as usize] = value;
            Ok(())
        }
    }

    fn peek_memory(&self, address: u16) -> Result<u32, MixMachineErr> {
        if (address >= mem_size) {
            Err(MixMachineErr{message: format!("Attempt to access invalid memory address {}.", address)})
        } else {
            Ok(self.memory[address as usize])
        }
    }

    fn poke_register(&mut self, reg: Register, value: u32) -> Result<(), MixMachineErr> {
        panic!("Not implemented.");
    }

    fn execute_load_op(&mut self, op: LoadOp) -> Result<(), MixMachineErr> {
        panic!("Not implemented.");
    }

    fn step(&mut self) -> Result<(), MixMachineErr> {
        // Try instruction fetch
        let instruction =
            if self.program_counter < mem_size {
                Ok(self.memory[self.program_counter as usize])
            } else {
                Err(MixMachineErr{message: format!("Attempted instruction fetch from invalid memory address {}.", self.program_counter)})
            };
        instruction.and_then(|instruction| {
            let op = Operation::from_u32(instruction);
            op.or_else(|x| Err(MixMachineErr{message: format!("Unknown or unimplemeted instruction: {}", instruction)}))
        }).and_then(|op| {
            match op {
                Load(op) => self.execute_load_op(op),
                _        => panic!("Not implemented."),
            }
        })
    }
}
