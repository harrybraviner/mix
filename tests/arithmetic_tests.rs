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
    assert_eq!(mix_machine.peek_overflow_toggle(), Ok(false));
}

#[test]
fn test_signed_addition() {
    // Adding two positive numbers together.
    // No indexing or field specifications.
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 5u8, 1u8)), Ok(())); // Add CONTENTS(10) to register A
    assert_eq!(mix_machine.poke_register(Register::RegA, 4u32 + (1u32 << 30)), Ok(()));    // Set register A to -4
    assert_eq!(mix_machine.poke_memory(10u16, 10u32), Ok(()));  // Set CONTENTS(10) to 10
    assert_eq!(mix_machine.step(), Ok(())); // Execute addition

    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(6u32));
    assert_eq!(mix_machine.peek_overflow_toggle(), Ok(false));
}

#[test]
fn test_addition_with_overflow() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 5u8, 1u8)), Ok(())); // Add CONTENTS(10) to register A
    assert_eq!(mix_machine.poke_register(Register::RegA, (1u32 << 30) - 2u32), Ok(()));    // Set register A to maximum int - 1
    assert_eq!(mix_machine.poke_memory(10u16, 3u32), Ok(()));  // Set CONTENTS(10) to 10
    assert_eq!(mix_machine.step(), Ok(())); // Execute addition
    
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(1u32));    // Knuth: "the remainder of the addition appearing in rA
                                                                        //         is as though a "1" had been carried into another
                                                                        //         register to the left of A."
    assert_eq!(mix_machine.peek_overflow_toggle(), Ok(true));
}
//
// Still to test - field specs, indexing, overflow with negative numbers, adding two largest
// numbers together
