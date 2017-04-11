extern crate mix;
use mix::mix_machine::*;
use mix::mix_operations::*;

#[test]
fn test_basic_addition() {
    // Adding two positive numbers together.
    // No indexing or field specifications.
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 5u8, 1u8)), Ok(())); // Add CONTENTS(10) to register A
    assert_eq!(mix_machine.poke_register(Register::RegA, 5u32), Ok(()));    // Set register A to 5
    assert_eq!(mix_machine.poke_memory(10u16, 10u32), Ok(()));  // Set CONTENTS(10) to 10
    assert_eq!(mix_machine.step(), Ok(())); // Execute addition

    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(15u32));
}
