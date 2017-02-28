use mix_operations::*;
use mix_operations::Operation::*;

const MAX_WORD_VALUE: u32 = (1 << 31) - 1;
const MEM_SIZE: u16 = 4000;

#[allow(non_snake_case)]    // Allow the register names to conform to Knuth's capitalisation
pub struct MixMachine {
    register_A: u32,
    register_X: u32,
    register_I1: u16,
    register_I2: u16,
    register_I3: u16,
    register_I4: u16,
    register_I5: u16,
    register_I6: u16,
    register_J: u16,
    program_counter: u16,   // Not strictly specified in MIX, but needed!
    memory: [u32; MEM_SIZE as usize]
}

#[derive(PartialEq, Debug)]
pub struct MixMachineErr {
    pub message: String,
}

#[derive(Clone, Copy)]
pub enum Register { RegA, RegX, RegI1, RegI2, RegI3, RegI4, RegI5, RegI6, RegJ } 

impl MixMachine {
    pub fn new() -> MixMachine {
        MixMachine{
            register_A: 0u32, register_X: 0u32,
            register_I1: 0u16, register_I2: 0u16,
            register_I3: 0u16, register_I4: 0u16,
            register_I5: 0u16, register_I6: 0u16,
            program_counter: 0u16,
            register_J: 0u16, memory: [0; MEM_SIZE as usize]
        }
    }

    pub fn poke_memory(&mut self, address: u16, value: u32) -> Result<(), MixMachineErr> {
        if address >= MEM_SIZE {
            Err(MixMachineErr{message: format!("Attempt to access invalid memory address {}.", address)})
        } else if value > MAX_WORD_VALUE {
            Err(MixMachineErr{message: format!("Attempt to write invalid value {}.", value)})
        } else {
            self.memory[address as usize] = value;
            Ok(())
        }
    }

    pub fn peek_memory(&self, address: u16) -> Result<u32, MixMachineErr> {
        if address >= MEM_SIZE {
            Err(MixMachineErr{message: format!("Attempt to access invalid memory address {}.", address)})
        } else {
            Ok(self.memory[address as usize])
        }
    }

    // Note: This function does not fail. It is up to the user to make sure that
    //       the value they are passive through is appropriate.
    fn poke_register(&mut self, reg: Register, value: u32) -> Result<(), MixMachineErr> {
        match reg {
            Register::RegA  => self.register_A  = value,
            Register::RegX  => self.register_X  = value,
            Register::RegI1 => self.register_I1 = MixMachine::reg32_to_reg16(value),
            Register::RegI2 => self.register_I2 = MixMachine::reg32_to_reg16(value),
            Register::RegI3 => self.register_I3 = MixMachine::reg32_to_reg16(value),
            Register::RegI4 => self.register_I4 = MixMachine::reg32_to_reg16(value),
            Register::RegI5 => self.register_I5 = MixMachine::reg32_to_reg16(value),
            Register::RegI6 => self.register_I6 = MixMachine::reg32_to_reg16(value),
            Register::RegJ  => self.register_J  = MixMachine::reg32_to_reg16(value),
        };
        Ok(())
    }

    pub fn peek_register(&self, reg: Register) -> Result<u32, MixMachineErr> {
        Ok(match reg {
            Register::RegA  => self.register_A,
            Register::RegX  => self.register_X,
            Register::RegI1 => MixMachine::reg16_to_reg32(self.register_I1),
            Register::RegI2 => MixMachine::reg16_to_reg32(self.register_I2),
            Register::RegI3 => MixMachine::reg16_to_reg32(self.register_I3),
            Register::RegI4 => MixMachine::reg16_to_reg32(self.register_I4),
            Register::RegI5 => MixMachine::reg16_to_reg32(self.register_I5),
            Register::RegI6 => MixMachine::reg16_to_reg32(self.register_I6),
            Register::RegJ  => MixMachine::reg16_to_reg32(self.register_J ),
        })
    }

    // For if we want to load a register from a memory value
    // FIXME - this should return some sort of error if bytes 1, 2, and 3 of reg32 are not all zero
    fn reg32_to_reg16 (reg32: u32) -> u16 {
        ((reg32 >> 18) + (reg32 % (1u32 << 12))) as u16
    }

    // For if we want to take a reg16 (I1, ..., I6 or J) and put it into
    // memory or a reg32.
    fn reg16_to_reg32 (reg16: u16) -> u32 {
        ((reg16 as u32) % (1u32 << 12)) + (((reg16 as u32) & (1u32 << 12)) << 18)
    }

    // FIXME - alter this to allow for negative base addresses
    fn compute_effective_address(&self, address: u16, index_spec: u8) -> Result<u16, MixMachineErr> {
        match index_spec {
            0 => Ok(address),
            1 => self.peek_register(Register::RegI1).map(|x| (x as u16) + address),
            2 => self.peek_register(Register::RegI2).map(|x| (x as u16) + address),
            3 => self.peek_register(Register::RegI3).map(|x| (x as u16) + address),
            4 => self.peek_register(Register::RegI4).map(|x| (x as u16) + address),
            5 => self.peek_register(Register::RegI5).map(|x| (x as u16) + address),
            6 => self.peek_register(Register::RegI6).map(|x| (x as u16) + address),
            _ => Err(MixMachineErr{message: format!("Invalid index_spec for computing effective address: {}", index_spec)}),
        }
    }

    fn negate_value(value: u32) -> u32 {
        value ^ (1<<30)
    }

    fn truncate_to_field(value: u32, field: u8) -> Result<u32, MixMachineErr> {
        let left  = field / 8;
        let right = field % 8;
        if right > 5 {
            Err(MixMachineErr{message: format!("Field specification {} has R={}. Must have R<=5.", field, right)})
        } else if left > 5 {
            Err(MixMachineErr{message: format!("Field specification {} has L={}. Must have L<=5.", field, left)})
        } else if left > right {
            Err(MixMachineErr{message: format!("Field specification {} has L={}, R={}. Must have L<=R.", field, left, right)})
        } else {
            let (left_byte, sign) = if left == 0 { (1, value & (1u32<<30)) } else { (left, 0u32) };
            let right_byte = if right == 0 { 1 } else { right };
            let bytes_out = if right == 0 {
                0u32
            } else {
                let bytes_in = value % (1u32<<30);
                (bytes_in >> (6*(5 - right_byte))) % (1 << (6*(right_byte - left_byte + 1)))
            };
            Ok(bytes_out + sign)
        }
    }

    fn execute_load_op(&mut self, op: &LoadOp) -> Result<(), MixMachineErr> {
        self.compute_effective_address(op.address, op.index_spec).and_then(|effective_address| {
            self.peek_memory(effective_address).and_then(|contents| {
                MixMachine::truncate_to_field(contents, op.field).and_then(|trunc_cont| {
                    if op.negative {
                        self.poke_register(op.register, MixMachine::negate_value(trunc_cont))
                    } else {
                        self.poke_register(op.register, trunc_cont)
                    }
                })
            })
        })
    }

    pub fn step(&mut self) -> Result<(), MixMachineErr> {
        // Try instruction fetch
        let instruction =
            if self.program_counter < MEM_SIZE {
                Ok(self.memory[self.program_counter as usize])
            } else {
                Err(MixMachineErr{message: format!("Attempted instruction fetch from invalid memory address {}.", self.program_counter)})
            };
        instruction.and_then(|instruction| {
            let op = Operation::from_u32(instruction);
            op.or_else(|_| Err(MixMachineErr{message: format!("Unknown or unimplemeted instruction: {}", instruction)}))
        }).and_then(|op| {
            match op {
                Load(op) => self.execute_load_op(&op),
                _        => panic!("Not implemented."),
            }.and_then(|_| {
                self.program_counter = self.program_counter + 1;
                Ok(())
            })
        })
    }
}

mod tests {
    use super::*;

    fn make_word(positive: bool, b1: u8, b2: u8, b3: u8, b4: u8, b5: u8) -> u32 {
        let max_byte = (1<<6) - 1;
        if    b1 > max_byte || b2 > max_byte || b3 > max_byte
           || b4 > max_byte || b5 > max_byte {
            panic!("Invalid bytes.")
        } else {
            let sgn = if positive { 0u32 } else { (1u32<<30) };
            sgn + ((b1 as u32)<<(6*4)) + ((b2 as u32)<<(6*3))
                + ((b3 as u32)<<(6*2)) + ((b4 as u32)<<(6*1)) + (b5 as u32)
        }
    }

    #[test]
    fn test_negate_value() {
        // 31 ones -> zero then 30 ones
        assert_eq!(MixMachine::negate_value((1u32<<31)-1), ((1u32<<30)-1));
        // zero then 30 ones -> 31 ones
        assert_eq!(MixMachine::negate_value((1u32<<30)-1), ((1u32<<31)-1));
    }

    #[test]
    fn test_truncate_to_field() {
        assert_eq!(Ok(make_word(false, 1u8, 2u8, 3u8, 4u8, 5u8)),
                   MixMachine::truncate_to_field(make_word(false, 1u8, 2u8, 3u8, 4u8, 5u8), 8*0 + 5));

        assert_eq!(Err(MixMachineErr{message: format!("Field specification {} has L=6. Must have L<=5.", 8*6 + 5)}),
                   MixMachine::truncate_to_field(make_word(false, 1u8, 2u8, 3u8, 4u8, 5u8), 8*6 + 5));

        assert_eq!(Err(MixMachineErr{message: format!("Field specification {} has R=6. Must have R<=5.", 8*0 + 6)}),
                   MixMachine::truncate_to_field(make_word(false, 1u8, 2u8, 3u8, 4u8, 5u8), 8*0 + 6));

        assert_eq!(Err(MixMachineErr{message: format!("Field specification {} has L=4, R=3. Must have L<=R.", 8*4 + 3)}),
                   MixMachine::truncate_to_field(make_word(false, 1u8, 2u8, 3u8, 4u8, 5u8), 8*4 + 3));


        assert_eq!(Ok(make_word(true, 1u8, 2u8, 3u8, 4u8, 5u8)),
                   MixMachine::truncate_to_field(make_word(false, 1u8, 2u8, 3u8, 4u8, 5u8), 8*1 + 5));
        assert_eq!(Ok(make_word(true, 1u8, 2u8, 3u8, 4u8, 5u8)),
                   MixMachine::truncate_to_field(make_word(true,  1u8, 2u8, 3u8, 4u8, 5u8), 8*1 + 5));

        assert_eq!(Ok(make_word(true, 0u8, 0u8, 3u8, 4u8, 5u8)),
                   MixMachine::truncate_to_field(make_word(true,  1u8, 2u8, 3u8, 4u8, 5u8), 8*3 + 5));

        assert_eq!(Ok(make_word(false, 0u8, 0u8, 1u8, 2u8, 3u8)),
                   MixMachine::truncate_to_field(make_word(false,  1u8, 2u8, 3u8, 4u8, 5u8), 8*0 + 3));

        assert_eq!(Ok(make_word(true, 0u8, 0u8, 0u8, 0u8, 4u8)),
                   MixMachine::truncate_to_field(make_word(false,  1u8, 2u8, 3u8, 4u8, 5u8), 8*4 + 4));

        assert_eq!(Ok(make_word(false, 0u8, 0u8, 0u8, 0u8, 0u8)),
                   MixMachine::truncate_to_field(make_word(false,  1u8, 2u8, 3u8, 4u8, 5u8), 8*0 + 0));

        assert_eq!(Ok(make_word(true, 0u8, 0u8, 0u8, 0u8, 1u8)),
                   MixMachine::truncate_to_field(make_word(false,  1u8, 2u8, 3u8, 4u8, 5u8), 8*1 + 1));
    }

    #[test]
    fn test_reg32_to_reg16() {
        assert_eq!(MixMachine::reg32_to_reg16(make_word(true,  0u8, 0u8, 0u8, 2u8, 5u8)), (5u16 + (2u16 << 6) + (0u16 << 12)));
        assert_eq!(MixMachine::reg32_to_reg16(make_word(false, 0u8, 0u8, 0u8, 2u8, 5u8)), (5u16 + (2u16 << 6) + (1u16 << 12)));
    }

    #[test]
    fn test_reg16_to_reg32() {
        assert_eq!((5u16 + (2u16 << 6) + (0u16 << 12)), MixMachine::reg32_to_reg16(make_word(true,  0u8, 0u8, 0u8, 2u8, 5u8)));
        assert_eq!((5u16 + (2u16 << 6) + (1u16 << 12)), MixMachine::reg32_to_reg16(make_word(false, 0u8, 0u8, 0u8, 2u8, 5u8)));
    }
        
}
