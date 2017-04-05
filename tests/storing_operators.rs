extern crate mix;

use mix::mix_machine::*;
use mix::mix_operations::*;

// Remember - the stored value should only replace the part of CONTENTS(M) specified by the field
// specification!

#[test]
fn test_store_op() {
    let mut mix_machine = MixMachine::new();
    // Store STX into memory
    mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 5u8, 31u8));
    // Set the register value appropriately
    mix_machine.poke_register(Register::RegX, 1234u32);

    // Execute the op to store register X
    assert_eq!(mix_machine.step(), Ok(()));

    // Check that the correct value really has been written
    assert_eq!(mix_machine.peek_memory(10u16), Ok(1234u32));
}

#[test]
fn test_store_op_field_32() {
    let mut mix_machine = MixMachine::new();
    // Store STX(2:3) into memory
    mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 8*2u8 + 3u8, 31u8));
    // Set the register value appropriately
    mix_machine.poke_register(Register::RegX, (1u32 << 24) + (2u32 << 18) + (3u32 << 12) + (4u32 << 6) + 5u32 );

    // Execute the op to store register X
    assert_eq!(mix_machine.step(), Ok(()));

    // Check that the correct value really has been written
    assert_eq!(mix_machine.peek_memory(10u16), Ok((0u32 << 24) + (4u32 << 18) + (5u32 << 12) + (0u32 << 6) + 0u32));
    
}

#[test]
fn test_store_op_field_16() {
    let mut mix_machine = MixMachine::new();
    // Store STI2(0:4) into memory
    mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 8*0u8 + 4u8, 26u8));
    // Set the register value appropriately
    assert_eq!(Ok(()), mix_machine.poke_register(Register::RegI2, (1u32 << 30) + (4u32 << 6) + 5u32));

    // Execute op to store register I2
    assert_eq!(mix_machine.step(), Ok(()));

    // Check that the correct value really has been written
    assert_eq!(mix_machine.peek_memory(10u16), Ok((1u32 << 30) + (4u32 << 12) + (5u32 << 6)));

}
