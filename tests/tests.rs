extern crate mix;

use std::cmp;
use mix::mix_machine::*;
use mix::mix_operations::*;

#[test]
fn test_load_op(){
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 5u8, 8u8)).and_then(|_| {; // LDA
                    mix_machine.poke_memory(10u16, 1234u32)}), Ok(())); // value to load
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(0u32));
    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(1234u32));
}

// Function to generate a test for given registers.
// Allows more complete test coverage.
fn test_load_with_offset_and_field_spec(reg_to_load: Register, reg_for_offset: Option<Register>,
                                        base_address: i16, offset_address: i16, value_to_load: u32,
                                        negative_load: bool, field_spec: u8) {
    let mut mix_machine = MixMachine::new();
    let opcode_2 = match reg_to_load {
        Register::RegA => 8,
        Register::RegX => 15,
        Register::RegI1 => 9,
        Register::RegI2 => 10,
        Register::RegI3 => 11,
        Register::RegI4 => 12,
        Register::RegI5 => 13,
        Register::RegI6 => 14,
        Register::RegJ => panic!("Register J cannot be loaded into!"),
    };
    let opcode_2 = if negative_load { opcode_2 + 8 } else { opcode_2 };
    let (opcode_1, index_spec_2) = match reg_for_offset {
        None => (9, 0u8),
        Some(reg_for_offset) => match reg_for_offset {
            Register::RegA => panic!("Register A cannot be used as an index register"),
            Register::RegX => panic!("Register X cannot be used as an index register"),
            Register::RegI1 => (9, 1u8),
            Register::RegI2 => (10, 2u8),
            Register::RegI3 => (11, 3u8),
            Register::RegI4 => (12, 4u8),
            Register::RegI5 => (13, 5u8),
            Register::RegI6 => (14, 6u8),
            Register::RegJ => panic!("Register J cannot be loaded into!"),
        },
    };
    // Setup the instruction code
    let base_address_positive: bool = base_address > 0;
    let base_address_abs = base_address.abs() as u16;

    // Load the offset into an index register
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 2u16, 0u8, 5u8, opcode_1)), Ok(()));
    // Load the value at the absolute address into a register
    assert_eq!(mix_machine.poke_memory(1u16, Operation::make_instruction(base_address_positive, base_address_abs, index_spec_2, field_spec, opcode_2)), Ok(()));
    // The actual value that will be loaded into the index register
    assert_eq!(mix_machine.poke_address_to_memory(2u16, offset_address), Ok(()));
    // The actual value that will be loaded into the target register
    let target_address = if reg_for_offset.is_some() {
        base_address + offset_address
    } else {
        base_address
    };
    assert_eq!(mix_machine.poke_memory((target_address) as u16, value_to_load), Ok(()));

    // Execute the load to the index register
    assert_eq!(mix_machine.step(), Ok(()));
    reg_for_offset.map(|reg_for_offset| {
        assert_eq!(mix_machine.peek_register(reg_for_offset), Ok(MixMachine::i32_to_reg32(offset_address as i32)));
    });
    // Execute the load to the main register
    assert_eq!(mix_machine.step(), Ok(()));
    let expected_output = {
        let spec_l = field_spec / 8u8; let spec_r = field_spec % 8u8;
        let sign_bit = if spec_l == 0u8 { value_to_load & (1u32 << 30) } else { 0u32 /* Positive */ };
        let mask = if spec_r == 0u8 {
            0u32
        } else {
            ((1u32 << (spec_r - cmp::max(1u8, spec_l) + 1)*6) - 1u32) << (5 - spec_r)*6
        };
        let field_masked_value = ((mask & value_to_load) >> (5 - spec_r)*6) + sign_bit;
        // Done with the field-spec bit - but were we told to negate the value we're loading?
        if negative_load { field_masked_value ^ (1u32 << 30) } else { field_masked_value } };
    assert_eq!(mix_machine.peek_register(reg_to_load), Ok(expected_output));


}

fn test_load_with_offset(reg_to_load: Register, reg_for_offset: Option<Register>,
                         base_address: i16, offset_address: i16, value_to_load: u32,
                         negative_load: bool) {
    test_load_with_offset_and_field_spec(reg_to_load, reg_for_offset, base_address, offset_address, value_to_load, negative_load, 5u8)
}


#[test]
fn test_load_op_with_no_offset() {
    test_load_with_offset(Register::RegX, None, 10i16, 0i16, (1u32<<8) + 345u32, false);
}

#[test]
fn test_load_op_with_offset() {
    test_load_with_offset(Register::RegX, Some(Register::RegI3), 10i16, 20i16, (1u32<<8) + 345u32, false);
}

#[test]
fn test_load_negative_op() {
    test_load_with_offset(Register::RegX, Some(Register::RegI3), 10i16, 0i16, (1u32<<8) + 345u32, true);
}

#[test]
fn test_load_op_with_negative_base_address() {
    test_load_with_offset(Register::RegX, Some(Register::RegI3), -10i16, 20i16, 1234u32, false);
}

#[test]
fn test_load_op_with_negative_offset_address() {
    test_load_with_offset(Register::RegX, Some(Register::RegI3), 10i16, -4i16, 1234u32, false);
}

#[test]
fn test_for_all_registers() {
    let value_to_load = 1234u32;
    let reg_set_for_index = vec![None, Some(Register::RegI1), Some(Register::RegI2), Some(Register::RegI3),
                                       Some(Register::RegI4), Some(Register::RegI5), Some(Register::RegI6)];
    let reg_set_for_load = vec![Register::RegX,  Register::RegA,  Register::RegI1, Register::RegI2,
                                Register::RegI3, Register::RegI4, Register::RegI5, Register::RegI6];
    let base_address_set   = vec![0i16, 100i16, -100i16];
    let offset_address_set = vec![0i16, 120i16, -20i16];
    for reg_for_index in &reg_set_for_index {
        for reg_to_load in &reg_set_for_load {
            for base_address in &base_address_set {
                for offset_address in &offset_address_set {
                    for negative_load in vec![false, true] {
                        if (reg_for_index.is_some() && (base_address + offset_address > 0i16))
                           || (*base_address > 0i16){
                            print!("Base: {}\nOffset: {}\n\n", base_address, offset_address);
                            test_load_with_offset(*reg_to_load, *reg_for_index, *base_address, *offset_address, value_to_load, negative_load);
                        }
                    }
                }
            }
        }
    }
}

 #[test]
fn test_load_with_truncated_field() {
    test_load_with_offset_and_field_spec(Register::RegX, None, 10i16, 20i16, 1234u32 + (1u32 << 30), false, 8*0 + 3);
    test_load_with_offset_and_field_spec(Register::RegX, None, 10i16, 20i16, 1234u32 + (1u32 << 30), false, 8*3 + 5);
}

#[test]
fn test_for_all_registers_and_field_specs() {
    let value_to_load = 1234u32;
    let reg_set_for_index = vec![None, Some(Register::RegI1), Some(Register::RegI2), Some(Register::RegI3),
                                       Some(Register::RegI4), Some(Register::RegI5), Some(Register::RegI6)];
    let reg_set_for_load = vec![Register::RegX,  Register::RegA,  Register::RegI1, Register::RegI2,
                                Register::RegI3, Register::RegI4, Register::RegI5, Register::RegI6];
    let field_spec_vals = vec![0u8, 1u8, 2u8, 3u8, 4u8, 5u8];
    let base_address_set   = vec![0i16, 100i16, -100i16];
    let offset_address_set = vec![0i16, 120i16, -20i16];
    for reg_for_index in &reg_set_for_index {
        for reg_to_load in &reg_set_for_load {
            for base_address in &base_address_set {
                for offset_address in &offset_address_set {
                    for negative_load in vec![false, true] {
                        if (reg_for_index.is_some() && (base_address + offset_address > 0i16))
                           || (*base_address > 0i16){
                               for spec_l in &field_spec_vals {
                                   for spec_r in &field_spec_vals {
                                       if spec_r >= spec_l {
                                            print!("Base: {}\nOffset: {}\n\n", base_address, offset_address);
                                            test_load_with_offset_and_field_spec(*reg_to_load, *reg_for_index, *base_address, *offset_address, value_to_load, negative_load, 8*spec_l + spec_r);
                                       }
                                   }
                               }
                        }
                    }
                }
            }
        }
    }
}
