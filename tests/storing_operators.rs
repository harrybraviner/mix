extern crate mix;

use mix::mix_machine::*;
use mix::mix_operations::*;

// Remember - the stored value should only replace the part of CONTENTS(M) specified by the field
// specification!

#[test]
fn test_store_op() {
    let mut mix_machine = MixMachine::new();
    // Program STX into memory
    mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 5u8, 31u8));
    // Set the register value appropriately
    mix_machine.poke_register(Register::RegX, 1234u32);

    // Execute the op to store register X
    assert_eq!(mix_machine.step(), Ok(()));

    // Check that the correct value really has been written
    assert_eq!(mix_machine.peek_memory(10u16), Ok(1234u32));
}
