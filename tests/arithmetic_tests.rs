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

#[test]
fn test_addition_of_negatives_with_overflow() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 5u8, 1u8)), Ok(())); // Add CONTENTS(10) to register A
    assert_eq!(mix_machine.poke_register(Register::RegA, (1u32 << 30) - 2u32 + (1u32 << 30)), Ok(()));    // Set register A to minimum int + 1
    assert_eq!(mix_machine.poke_memory(10u16, 4u32 + (1u32 << 30)), Ok(()));  // Set CONTENTS(10) to -4
    assert_eq!(mix_machine.step(), Ok(())); // Execute addition
    
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(2u32 + (1u32 << 30)));     // Knuth: "the remainder of the addition appearing in rA
                                                                                        //         is as though a "1" had been carried into another
                                                                                        //         register to the left of A."
    assert_eq!(mix_machine.peek_overflow_toggle(), Ok(true));
}

#[test]
fn test_addition_near_overflow() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 5u8, 1u8)), Ok(())); // Add CONTENTS(10) to register A
    assert_eq!(mix_machine.poke_register(Register::RegA, (1u32 << 30) - 3u32), Ok(()));    // Set register A to maximum int - 2
    assert_eq!(mix_machine.poke_memory(10u16, 2u32), Ok(()));  // Set CONTENTS(10) to 2
    assert_eq!(mix_machine.step(), Ok(())); // Execute addition
    
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok((1u32 << 30) - 1));
    assert_eq!(mix_machine.peek_overflow_toggle(), Ok(false));
}

#[test]
fn test_addition_of_negatives_near_overflow() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 5u8, 1u8)), Ok(())); // Add CONTENTS(10) to register A
    assert_eq!(mix_machine.poke_register(Register::RegA, (1u32 << 30) - 3u32 + (1u32 << 30)), Ok(()));    // Set register A to minimum int + 2
    assert_eq!(mix_machine.poke_memory(10u16, 2u32 + (1u32 << 30)), Ok(()));  // Set CONTENTS(10) to -2
    assert_eq!(mix_machine.step(), Ok(())); // Execute addition
    
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok((1u32 << 30) - 1 + (1u32 << 30)));
    assert_eq!(mix_machine.peek_overflow_toggle(), Ok(false));
}

#[test]
fn test_addition_with_indexing() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 2u8, 5u8, 1u8)), Ok(())); // Add CONTENTS(10+rI2) to register A
    assert_eq!(mix_machine.poke_register(Register::RegI2, 2u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegA, 23u32), Ok(()));    // Set register A to 23
    assert_eq!(mix_machine.poke_memory(12u16, 9u32), Ok(()));  // Set CONTENTS(12) to 9
    assert_eq!(mix_machine.step(), Ok(())); // Execute addition
    
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(32u32));
    assert_eq!(mix_machine.peek_overflow_toggle(), Ok(false));
}

#[test]
fn test_addition_with_field_spec() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 8u8 + 5u8, 1u8)), Ok(())); // Add CONTENTS(10)(1:5) to register A
    assert_eq!(mix_machine.poke_register(Register::RegA, 23u32), Ok(()));    // Set register A to 23
    assert_eq!(mix_machine.poke_memory(10u16, 9u32 + (1u32 << 30)), Ok(()));  // Set CONTENTS(10) to -9
    assert_eq!(mix_machine.step(), Ok(())); // Execute addition
    
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(32u32));   // Note: 23 + 9, not 23 - 9, because the sign bit has been removed
    assert_eq!(mix_machine.peek_overflow_toggle(), Ok(false));
}

#[test]
fn test_addition_of_two_largest_numbers() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 5u8, 1u8)), Ok(())); // Add CONTENTS(10) to register A
    assert_eq!(mix_machine.poke_register(Register::RegA, (1u32 << 30) - 1u32), Ok(()));    // Set register A to maximum int
    assert_eq!(mix_machine.poke_memory(10u16, (1u32 << 30) - 1u32), Ok(()));  // Set CONTENTS(10) to maximum int
    assert_eq!(mix_machine.step(), Ok(())); // Execute addition
    
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok((1u32 << 30) - 2u32));   // Note: maximum int - 1
    assert_eq!(mix_machine.peek_overflow_toggle(), Ok(true));
}
