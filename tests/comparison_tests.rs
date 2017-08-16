extern crate mix;
use mix::mix_machine::*;
use mix::mix_operations::*;

#[test]
fn compare_a_less() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 5u8, 56u8)), Ok(()));    // CMPA 10
    assert_eq!(mix_machine.poke_memory(1u16, Operation::make_instruction(true, 11u16, 0u8, 5u8, 56u8)), Ok(()));    // CMPA 11
    assert_eq!(mix_machine.poke_memory(2u16, Operation::make_instruction(true, 12u16, 0u8, 5u8, 56u8)), Ok(()));    // CMPA 12
    assert_eq!(mix_machine.poke_memory(10u16, 5u32), Ok(()));
    assert_eq!(mix_machine.poke_memory(11u16, 2u32), Ok(()));
    assert_eq!(mix_machine.poke_memory(12u16, 1u32 + (1u32 << 30)), Ok(()));

    assert_eq!(mix_machine.poke_register(Register::RegA, 4u32), Ok(()));
    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Less));

    assert_eq!(mix_machine.poke_register(Register::RegA, 4u32 + (1u32 << 30)), Ok(()));
    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Less));

    assert_eq!(mix_machine.poke_register(Register::RegA, 4u32 + (1u32 << 30)), Ok(()));
    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Less));
}

#[test]
fn compare_a_greater() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 5u8, 56u8)), Ok(()));    // CMPA 10
    assert_eq!(mix_machine.poke_memory(1u16, Operation::make_instruction(true, 11u16, 0u8, 5u8, 56u8)), Ok(()));    // CMPA 11
    assert_eq!(mix_machine.poke_memory(2u16, Operation::make_instruction(true, 12u16, 0u8, 5u8, 56u8)), Ok(()));    // CMPA 12
    assert_eq!(mix_machine.poke_memory(10u16, 5u32), Ok(()));
    assert_eq!(mix_machine.poke_memory(11u16, 2u32), Ok(()));
    assert_eq!(mix_machine.poke_memory(12u16, 5u32 + (1u32 << 30)), Ok(()));

    assert_eq!(mix_machine.poke_register(Register::RegA, 6u32), Ok(()));
    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Greater));

    assert_eq!(mix_machine.poke_register(Register::RegA, 4u32), Ok(()));
    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Greater));

    assert_eq!(mix_machine.poke_register(Register::RegA, 4u32 + (1u32 << 30)), Ok(()));
    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Greater));
}

#[test]
fn compare_a_equal() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 5u8, 56u8)), Ok(()));    // CMPA 10
    assert_eq!(mix_machine.poke_memory(1u16, Operation::make_instruction(true, 11u16, 0u8, 5u8, 56u8)), Ok(()));    // CMPA 11
    assert_eq!(mix_machine.poke_memory(2u16, Operation::make_instruction(true, 12u16, 0u8, 5u8, 56u8)), Ok(()));    // CMPA 12
    assert_eq!(mix_machine.poke_memory(3u16, Operation::make_instruction(true, 11u16, 0u8, 5u8, 56u8)), Ok(()));    // CMPA 11
    assert_eq!(mix_machine.poke_memory(4u16, Operation::make_instruction(true, 12u16, 0u8, 5u8, 56u8)), Ok(()));    // CMPA 12
    assert_eq!(mix_machine.poke_memory(5u16, Operation::make_instruction(true, 13u16, 0u8, 5u8, 56u8)), Ok(()));    // CMPA 13
    assert_eq!(mix_machine.poke_memory(10u16, 5u32), Ok(()));
    assert_eq!(mix_machine.poke_memory(11u16, 0u32), Ok(()));
    assert_eq!(mix_machine.poke_memory(12u16, 0u32 + (1u32 << 30)), Ok(()));
    assert_eq!(mix_machine.poke_memory(13u16, 5u32 + (1u32 << 30)), Ok(()));

    assert_eq!(mix_machine.poke_register(Register::RegA, 5u32), Ok(()));
    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Equal));

    // Next four are all the +/-0 combinations
    assert_eq!(mix_machine.poke_register(Register::RegA, 0u32), Ok(()));
    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Equal));
    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Equal));

    assert_eq!(mix_machine.poke_register(Register::RegA, 0u32 + (1u32 << 30)), Ok(()));
    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Equal));
    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Equal));

    assert_eq!(mix_machine.poke_register(Register::RegA, 5u32 + (1u32 << 30)), Ok(()));
    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Equal));
}

#[test]
fn compare_a_with_indexing() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 1u8, 5u8, 56u8)), Ok(()));    // CMPA 10,1
    assert_eq!(mix_machine.poke_register(Register::RegI1, 5u32), Ok(()));    // Offset of 5 for address
    assert_eq!(mix_machine.poke_memory(1u16, Operation::make_instruction(true, 10u16, 2u8, 5u8, 56u8)), Ok(()));    // CMPA 10,2
    assert_eq!(mix_machine.poke_register(Register::RegI2, 10u32), Ok(()));    // Offset of 10 for address
    assert_eq!(mix_machine.poke_memory(15u16, 10u32), Ok(()));
    assert_eq!(mix_machine.poke_memory(20u16, 10u32 + (1u32 << 30)), Ok(()));

    assert_eq!(mix_machine.poke_register(Register::RegA, 6u32), Ok(()));

    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Less));

    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Greater));
}
 
#[test]
fn compare_a_with_field_spec() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 8u8 + 5u8, 56u8)), Ok(()));    // CMPA 10,(1:5)
    assert_eq!(mix_machine.poke_memory(1u16, Operation::make_instruction(true, 10u16, 0u8, 8u8 + 5u8, 56u8)), Ok(()));    // CMPA 10,(1:5)
    assert_eq!(mix_machine.poke_memory(2u16, Operation::make_instruction(true, 10u16, 0u8, 3u8, 56u8)), Ok(()));    // CMPA 10,(0:3)
    assert_eq!(mix_machine.poke_memory(3u16, Operation::make_instruction(true, 10u16, 0u8, 3u8, 56u8)), Ok(()));    // CMPA 10,(0:3)
    
    assert_eq!(mix_machine.poke_memory(10u16, 10u32), Ok(()));

    assert_eq!(mix_machine.poke_register(Register::RegA, 11u32 + (1u32 << 30)), Ok(()));
    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Greater)); // |-11| > |10|

    assert_eq!(mix_machine.poke_register(Register::RegA, 10u32), Ok(()));
    assert_eq!(mix_machine.poke_memory(10u16, 11u32 + (1u32 << 30)), Ok(()));
    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Less)); // |10| < |-11|

    assert_eq!(mix_machine.poke_register(Register::RegA, (10u32 << 12) + 10u32), Ok(()));
    assert_eq!(mix_machine.poke_memory(10u16, (10u32 << 12) + 0u32), Ok(()));
    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Equal));    // The 10 in the register is ignored

    assert_eq!(mix_machine.poke_register(Register::RegA, (10u32 << 12) + 10u32 + (1u32 << 30)), Ok(()));
    assert_eq!(mix_machine.poke_memory(10u16, (10u32 << 12) + 0u32), Ok(()));
    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Less));    // Checking thaet the sign bit is still considered
}
