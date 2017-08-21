extern crate mix;
use mix::mix_operations::*;
use mix::mix_machine::*;

#[test]
fn jump() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 5u16, 1u8, 0u8, 39u8)), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegI1, 3u32 + (1u32 << 30)), Ok(()));    // 5 - 3 = 2
    assert_eq!(mix_machine.poke_memory(1u16, Operation::make_instruction(true, 5u16, 0u8, 2u8, 48u8)), Ok(()));
    assert_eq!(mix_machine.poke_memory(2u16, Operation::make_instruction(true, 15u16, 0u8, 2u8, 48u8)), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegJ, 20u32), Ok(()));   // Just to check that this does indeed get set back to zero

    // Test that we jump to location 2, and hence insert 15 rather than 5 into rA
    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(15u32));
    assert_eq!(mix_machine.peek_register(Register::RegJ), Ok(1u32));
}

#[test]
fn jump_save_j() {
    // Should jump without changing the contents of rJ
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 5u16, 1u8, 1u8, 39u8)), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegI1, 3u32 + (1u32 << 30)), Ok(()));    // 5 - 3 = 2
    assert_eq!(mix_machine.poke_memory(1u16, Operation::make_instruction(true, 5u16, 0u8, 2u8, 48u8)), Ok(()));
    assert_eq!(mix_machine.poke_memory(2u16, Operation::make_instruction(true, 15u16, 0u8, 2u8, 48u8)), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegJ, 20u32), Ok(()));   // Just to check that this does indeed get set back to zero

    // Test that we jump to location 2, and hence insert 15 rather than 5 into rA
    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(15u32));
    assert_eq!(mix_machine.peek_register(Register::RegJ), Ok(20u32));
}

#[test]
fn jump_if_overflow() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 5u8, 8u8)), Ok(())); // LDA 10
    assert_eq!(mix_machine.poke_memory(10u16, (1u32 << 30) - 1u32), Ok(())); // One less than an overflow
    assert_eq!(mix_machine.poke_memory(1u16, Operation::make_instruction(true, 1u16, 0u8, 0u8, 48u8)), Ok(())); // INCA 1
    assert_eq!(mix_machine.poke_memory(2u16, Operation::make_instruction(true, 4u16, 0u8, 2u8, 39u8)), Ok(())); // JOV 4
    assert_eq!(mix_machine.poke_memory(3u16, Operation::make_instruction(true, 20u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 20
    assert_eq!(mix_machine.poke_memory(4u16, Operation::make_instruction(true, 40u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 40
    assert_eq!(mix_machine.poke_memory(5u16, Operation::make_instruction(true, 3u16, 0u8, 2u8, 39u8)), Ok(())); // JOV 3
    assert_eq!(mix_machine.poke_memory(6u16, Operation::make_instruction(true, 60u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 60


    assert_eq!(mix_machine.step(), Ok(())); // Load 2^30 - 1 into rA
    assert_eq!(mix_machine.step(), Ok(())); // Increment rA, cause an overflow
    assert_eq!(mix_machine.peek_overflow_toggle(), Ok(true));
    assert_eq!(mix_machine.step(), Ok(())); // Cause the jump
    assert_eq!(mix_machine.peek_overflow_toggle(), Ok(false));  // Check that the overflow has been cleared
    assert_eq!(mix_machine.step(), Ok(())); // Execute instruction at 4 (ENTA 40);
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(40u32));   // Check that the appropriate value was loaded
    assert_eq!(mix_machine.step(), Ok(())); // Execute instruction at 5 (JOV 3, should not jump because overflow is now cleared);
    assert_eq!(mix_machine.step(), Ok(())); // Execute instruction following 5 (should be 6, not 3)
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(60u32));   // Check that the appropriate value was loaded
}


#[test]
fn jump_no_overflow() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 5u8, 8u8)), Ok(())); // LDA 10
    assert_eq!(mix_machine.poke_memory(10u16, (1u32 << 30) - 1u32), Ok(())); // One less than an overflow
    assert_eq!(mix_machine.poke_memory(1u16, Operation::make_instruction(true, 1u16, 0u8, 0u8, 48u8)), Ok(())); // INCA 1
    assert_eq!(mix_machine.poke_memory(2u16, Operation::make_instruction(true, 4u16, 0u8, 3u8, 39u8)), Ok(())); // JNOV 4
    assert_eq!(mix_machine.poke_memory(3u16, Operation::make_instruction(true, 20u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 20
    assert_eq!(mix_machine.poke_memory(4u16, Operation::make_instruction(true, 40u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 40
    assert_eq!(mix_machine.poke_memory(5u16, Operation::make_instruction(true, 3u16, 0u8, 3u8, 39u8)), Ok(())); // JNOV 3
    assert_eq!(mix_machine.poke_memory(6u16, Operation::make_instruction(true, 60u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 60


    assert_eq!(mix_machine.step(), Ok(())); // Load 2^30 - 1 into rA
    assert_eq!(mix_machine.step(), Ok(())); // Increment rA, cause an overflow
    assert_eq!(mix_machine.peek_overflow_toggle(), Ok(true));
    assert_eq!(mix_machine.step(), Ok(())); // No jump should happen
    assert_eq!(mix_machine.peek_overflow_toggle(), Ok(false));  // Check that the overflow has been cleared
    assert_eq!(mix_machine.step(), Ok(())); // Execute instruction at 3 (ENTA 20);
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(20u32));   // Check that the appropriate value was loaded
    assert_eq!(mix_machine.step(), Ok(())); // Execute instruction at 4 (ENTA 40)
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(40u32));   // Check that the appropriate value was loaded
    assert_eq!(mix_machine.step(), Ok(())); // Execute instruction at 5 (JNOV 3, should jump because overflow is now cleared);
    assert_eq!(mix_machine.step(), Ok(())); // Execute instruction following 5 (should be 3, not 6)
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(20u32));   // Check that the appropriate value was loaded
}

#[test]
fn jump_less() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 10
    assert_eq!(mix_machine.poke_memory(100u16, 11u32), Ok(())); // Put 11 into location 100
    assert_eq!(mix_machine.poke_memory(1u16, Operation::make_instruction(true, 100u16, 0u8, 5u8, 56u8)), Ok(())); // CMPA 100
    assert_eq!(mix_machine.poke_memory(2u16, Operation::make_instruction(true, 4u16, 0u8, 4u8, 39u8)), Ok(())); // JL 4
    assert_eq!(mix_machine.poke_memory(3u16, Operation::make_instruction(true, 50u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 50
    assert_eq!(mix_machine.poke_memory(4u16, Operation::make_instruction(true, 60u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 60
    assert_eq!(mix_machine.poke_memory(5u16, Operation::make_instruction(true, 100u16, 0u8, 5u8, 56u8)), Ok(())); // CMPA 100
    assert_eq!(mix_machine.poke_memory(6u16, Operation::make_instruction(true, 4u16, 0u8, 4u8, 39u8)), Ok(())); // JL 4
    assert_eq!(mix_machine.poke_memory(7u16, Operation::make_instruction(true, 11u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 11
    assert_eq!(mix_machine.poke_memory(8u16, Operation::make_instruction(true, 100u16, 0u8, 5u8, 56u8)), Ok(())); // CMPA 100
    assert_eq!(mix_machine.poke_memory(9u16, Operation::make_instruction(true, 4u16, 0u8, 4u8, 39u8)), Ok(())); // JL 4
    assert_eq!(mix_machine.poke_memory(10u16, Operation::make_instruction(true, 12u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 12

    assert_eq!(mix_machine.step(), Ok(())); // Enter 10 into rA
    assert_eq!(mix_machine.step(), Ok(())); // Comparison - should produce Less
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Less));
    assert_eq!(mix_machine.step(), Ok(())); // Jump - should jump to 4
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 60
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(60u32));
    assert_eq!(mix_machine.step(), Ok(())); // Comparison - should produce Greater
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Greater));
    assert_eq!(mix_machine.step(), Ok(())); // JL4 - should not jump this time
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 11
    assert_eq!(mix_machine.step(), Ok(())); // Comparison - should produce Equal
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Equal));
    assert_eq!(mix_machine.step(), Ok(())); // JL4 - should not jump this time
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 12
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(12u32));
}

#[test]
fn jump_equal() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 10
    assert_eq!(mix_machine.poke_memory(100u16, 10u32), Ok(())); // Put 10 into location 100
    assert_eq!(mix_machine.poke_memory(1u16, Operation::make_instruction(true, 100u16, 0u8, 5u8, 56u8)), Ok(())); // CMPA 100
    assert_eq!(mix_machine.poke_memory(2u16, Operation::make_instruction(true, 4u16, 0u8, 5u8, 39u8)), Ok(())); // JE 4
    assert_eq!(mix_machine.poke_memory(3u16, Operation::make_instruction(true, 50u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 50
    assert_eq!(mix_machine.poke_memory(4u16, Operation::make_instruction(true, 60u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 60
    assert_eq!(mix_machine.poke_memory(5u16, Operation::make_instruction(true, 100u16, 0u8, 5u8, 56u8)), Ok(())); // CMPA 100
    assert_eq!(mix_machine.poke_memory(6u16, Operation::make_instruction(true, 4u16, 0u8, 5u8, 39u8)), Ok(())); // JE 4
    assert_eq!(mix_machine.poke_memory(7u16, Operation::make_instruction(true, 9u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 9
    assert_eq!(mix_machine.poke_memory(8u16, Operation::make_instruction(true, 100u16, 0u8, 5u8, 56u8)), Ok(())); // CMPA 100
    assert_eq!(mix_machine.poke_memory(9u16, Operation::make_instruction(true, 4u16, 0u8, 5u8, 39u8)), Ok(())); // JE 4
    assert_eq!(mix_machine.poke_memory(10u16, Operation::make_instruction(true, 12u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 12

    assert_eq!(mix_machine.step(), Ok(())); // Enter 10 into rA
    assert_eq!(mix_machine.step(), Ok(())); // Comparison - should produce Equal
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Equal));
    assert_eq!(mix_machine.step(), Ok(())); // Jump - should jump to 4
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 60
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(60u32));
    assert_eq!(mix_machine.step(), Ok(())); // Comparison - should produce Greater
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Greater));
    assert_eq!(mix_machine.step(), Ok(())); // JE4 - should not jump this time
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 9
    assert_eq!(mix_machine.step(), Ok(())); // Comparison - should produce Less
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Less));
    assert_eq!(mix_machine.step(), Ok(())); // JE4 - should not jump this time
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 12
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(12u32));
}

#[test]
fn jump_greater() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 10
    assert_eq!(mix_machine.poke_memory(100u16, 9u32), Ok(())); // Put 9 into location 100
    assert_eq!(mix_machine.poke_memory(1u16, Operation::make_instruction(true, 100u16, 0u8, 5u8, 56u8)), Ok(())); // CMPA 100
    assert_eq!(mix_machine.poke_memory(2u16, Operation::make_instruction(true, 4u16, 0u8, 6u8, 39u8)), Ok(())); // JG 4
    assert_eq!(mix_machine.poke_memory(3u16, Operation::make_instruction(true, 50u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 50
    assert_eq!(mix_machine.poke_memory(4u16, Operation::make_instruction(true, 2u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 2
    assert_eq!(mix_machine.poke_memory(5u16, Operation::make_instruction(true, 100u16, 0u8, 5u8, 56u8)), Ok(())); // CMPA 100
    assert_eq!(mix_machine.poke_memory(6u16, Operation::make_instruction(true, 4u16, 0u8, 6u8, 39u8)), Ok(())); // JG 4
    assert_eq!(mix_machine.poke_memory(7u16, Operation::make_instruction(true, 9u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 9
    assert_eq!(mix_machine.poke_memory(8u16, Operation::make_instruction(true, 100u16, 0u8, 5u8, 56u8)), Ok(())); // CMPA 100
    assert_eq!(mix_machine.poke_memory(9u16, Operation::make_instruction(true, 4u16, 0u8, 6u8, 39u8)), Ok(())); // JG 4
    assert_eq!(mix_machine.poke_memory(10u16, Operation::make_instruction(true, 12u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 12

    assert_eq!(mix_machine.step(), Ok(())); // Enter 10 into rA
    assert_eq!(mix_machine.step(), Ok(())); // Comparison - should produce Greater
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Greater));
    assert_eq!(mix_machine.step(), Ok(())); // Jump - should jump to 4
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 2
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(2u32));
    assert_eq!(mix_machine.step(), Ok(())); // Comparison - should produce Less
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Less));
    assert_eq!(mix_machine.step(), Ok(())); // JG4 - should not jump this time
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 9
    assert_eq!(mix_machine.step(), Ok(())); // Comparison - should produce Equal
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Equal));
    assert_eq!(mix_machine.step(), Ok(())); // JG4 - should not jump this time
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 12
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(12u32));
}

#[test]
fn jump_greater_or_equal() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 10
    assert_eq!(mix_machine.poke_memory(100u16, 10u32), Ok(())); // Put 10 into location 100
    assert_eq!(mix_machine.poke_memory(1u16, Operation::make_instruction(true, 100u16, 0u8, 5u8, 56u8)), Ok(())); // CMPA 100
    assert_eq!(mix_machine.poke_memory(2u16, Operation::make_instruction(true, 4u16, 0u8, 7u8, 39u8)), Ok(())); // JGE 4
    assert_eq!(mix_machine.poke_memory(3u16, Operation::make_instruction(true, 50u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 50
    assert_eq!(mix_machine.poke_memory(4u16, Operation::make_instruction(true, 5u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 5
    assert_eq!(mix_machine.poke_memory(5u16, Operation::make_instruction(true, 100u16, 0u8, 5u8, 56u8)), Ok(())); // CMPA 100
    assert_eq!(mix_machine.poke_memory(6u16, Operation::make_instruction(true, 4u16, 0u8, 7u8, 39u8)), Ok(())); // JGE 4
    assert_eq!(mix_machine.poke_memory(7u16, Operation::make_instruction(true, 10u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 10
    assert_eq!(mix_machine.poke_memory(8u16, Operation::make_instruction(true, 100u16, 0u8, 5u8, 56u8)), Ok(())); // CMPA 100
    assert_eq!(mix_machine.poke_memory(9u16, Operation::make_instruction(true, 4u16, 0u8, 7u8, 39u8)), Ok(())); // JGE 4
    assert_eq!(mix_machine.poke_memory(10u16, Operation::make_instruction(true, 12u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 12

    assert_eq!(mix_machine.step(), Ok(())); // Enter 10 into rA
    assert_eq!(mix_machine.step(), Ok(())); // Comparison - should produce Equal
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Equal));
    assert_eq!(mix_machine.step(), Ok(())); // Jump - should jump to 4
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 5
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(5u32));
    assert_eq!(mix_machine.step(), Ok(())); // Comparison - should produce Less
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Less));
    assert_eq!(mix_machine.step(), Ok(())); // JGE4 - should not jump this time
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 10
    assert_eq!(mix_machine.step(), Ok(())); // Comparison - should produce Equal
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Equal));
    assert_eq!(mix_machine.step(), Ok(())); // JGE4 - should jump
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 5
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(5u32));
}

#[test]
fn jump_not_equal() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 20u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 20
    assert_eq!(mix_machine.poke_memory(100u16, 11u32), Ok(())); // Put 11 into location 100
    assert_eq!(mix_machine.poke_memory(1u16, Operation::make_instruction(true, 100u16, 0u8, 5u8, 56u8)), Ok(())); // CMPA 100
    assert_eq!(mix_machine.poke_memory(2u16, Operation::make_instruction(true, 4u16, 0u8, 8u8, 39u8)), Ok(())); // JNE 4
    assert_eq!(mix_machine.poke_memory(3u16, Operation::make_instruction(true, 50u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 50
    assert_eq!(mix_machine.poke_memory(4u16, Operation::make_instruction(true, 11u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 11
    assert_eq!(mix_machine.poke_memory(5u16, Operation::make_instruction(true, 100u16, 0u8, 5u8, 56u8)), Ok(())); // CMPA 100
    assert_eq!(mix_machine.poke_memory(6u16, Operation::make_instruction(true, 4u16, 0u8, 8u8, 39u8)), Ok(())); // JNE 4
    assert_eq!(mix_machine.poke_memory(7u16, Operation::make_instruction(true, 10u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 10
    assert_eq!(mix_machine.poke_memory(8u16, Operation::make_instruction(true, 100u16, 0u8, 5u8, 56u8)), Ok(())); // CMPA 100
    assert_eq!(mix_machine.poke_memory(9u16, Operation::make_instruction(true, 4u16, 0u8, 8u8, 39u8)), Ok(())); // JNE 4
    assert_eq!(mix_machine.poke_memory(10u16, Operation::make_instruction(true, 12u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 12

    assert_eq!(mix_machine.step(), Ok(())); // Enter 20 into rA
    assert_eq!(mix_machine.step(), Ok(())); // Comparison - should produce Greater
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Greater));
    assert_eq!(mix_machine.step(), Ok(())); // Jump - should jump to 4
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 11
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(11u32));
    assert_eq!(mix_machine.step(), Ok(())); // Comparison - should produce Equal
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Equal));
    assert_eq!(mix_machine.step(), Ok(())); // JNE4 - should not jump this time
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 10
    assert_eq!(mix_machine.step(), Ok(())); // Comparison - should produce Less
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Less));
    assert_eq!(mix_machine.step(), Ok(())); // JNE4 - should jump
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 11
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(11u32));
}

#[test]
fn jump_less_or_equal() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 11u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 11
    assert_eq!(mix_machine.poke_memory(100u16, 11u32), Ok(())); // Put 11 into location 100
    assert_eq!(mix_machine.poke_memory(1u16, Operation::make_instruction(true, 100u16, 0u8, 5u8, 56u8)), Ok(())); // CMPA 100
    assert_eq!(mix_machine.poke_memory(2u16, Operation::make_instruction(true, 4u16, 0u8, 9u8, 39u8)), Ok(())); // JLE 4
    assert_eq!(mix_machine.poke_memory(3u16, Operation::make_instruction(true, 50u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 50
    assert_eq!(mix_machine.poke_memory(4u16, Operation::make_instruction(true, 20u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 20
    assert_eq!(mix_machine.poke_memory(5u16, Operation::make_instruction(true, 100u16, 0u8, 5u8, 56u8)), Ok(())); // CMPA 100
    assert_eq!(mix_machine.poke_memory(6u16, Operation::make_instruction(true, 4u16, 0u8, 9u8, 39u8)), Ok(())); // JLE 4
    assert_eq!(mix_machine.poke_memory(7u16, Operation::make_instruction(true, 10u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 10
    assert_eq!(mix_machine.poke_memory(8u16, Operation::make_instruction(true, 100u16, 0u8, 5u8, 56u8)), Ok(())); // CMPA 100
    assert_eq!(mix_machine.poke_memory(9u16, Operation::make_instruction(true, 4u16, 0u8, 9u8, 39u8)), Ok(())); // JLE 4
    assert_eq!(mix_machine.poke_memory(10u16, Operation::make_instruction(true, 12u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 12

    assert_eq!(mix_machine.step(), Ok(())); // Enter 11 into rA
    assert_eq!(mix_machine.step(), Ok(())); // Comparison - should produce Equal
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Equal));
    assert_eq!(mix_machine.step(), Ok(())); // Jump - should jump to 4
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 2
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(20u32));
    assert_eq!(mix_machine.step(), Ok(())); // Comparison - should produce Greater
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Greater));
    assert_eq!(mix_machine.step(), Ok(())); // JLE4 - should not jump this time
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 10
    assert_eq!(mix_machine.step(), Ok(())); // Comparison - should produce Less
    assert_eq!(mix_machine.peek_comparison_indicator(), Ok(ComparisonState::Less));
    assert_eq!(mix_machine.step(), Ok(())); // JLE4 - should jump
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 20
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(20u32));
}

// Tests below here (for sign of registers) are less extensive
// since there's considerable code re-use with the above.

#[test]
fn jump_a_negative() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_register(Register::RegA, 10u32), Ok(()));
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 2u16, 0u8, 0u8, 40u8)), Ok(())); // JAN 2
    assert_eq!(mix_machine.poke_memory(1u16, Operation::make_instruction(true, 10u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 10
    assert_eq!(mix_machine.poke_memory(2u16, Operation::make_instruction(true, 20u16, 0u8, 3u8, 48u8)), Ok(())); // ENNA 20
    assert_eq!(mix_machine.poke_memory(3u16, Operation::make_instruction(true, 5u16, 0u8, 0u8, 40u8)), Ok(())); // JAN 5
    assert_eq!(mix_machine.poke_memory(4u16, Operation::make_instruction(true, 30u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 10
    assert_eq!(mix_machine.poke_memory(5u16, Operation::make_instruction(true, 0u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 0
    assert_eq!(mix_machine.poke_memory(6u16, Operation::make_instruction(true, 2u16, 0u8, 0u8, 40u8)), Ok(())); // JAN 2
    assert_eq!(mix_machine.poke_memory(7u16, Operation::make_instruction(true, 5u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 5

    assert_eq!(mix_machine.step(), Ok(())); // JAN 2 - should not jump since A is positive
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 10
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(10u32));
    assert_eq!(mix_machine.step(), Ok(())); // ENNA 20 (i.e. -20)
    assert_eq!(mix_machine.step(), Ok(())); // JAN 5
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 0
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(0u32));
    assert_eq!(mix_machine.step(), Ok(())); // JAN 2 - should not jump since A is zero
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 5
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(5u32));
}

#[test]
fn jump_x_positive() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_register(Register::RegX, 10u32), Ok(()));
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 2u16, 0u8, 2u8, 47u8)), Ok(())); // JXP 2
    assert_eq!(mix_machine.poke_memory(1u16, Operation::make_instruction(true, 10u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 10
    assert_eq!(mix_machine.poke_memory(2u16, Operation::make_instruction(true, 20u16, 0u8, 3u8, 48u8)), Ok(())); // ENNA 20
    assert_eq!(mix_machine.poke_memory(3u16, Operation::make_instruction(true, 20u16, 0u8, 3u8, 55u8)), Ok(())); // ENNX 20
    assert_eq!(mix_machine.poke_memory(4u16, Operation::make_instruction(true, 5u16, 0u8, 2u8, 47u8)), Ok(())); // JXP 6
    assert_eq!(mix_machine.poke_memory(5u16, Operation::make_instruction(true, 10u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 10
    assert_eq!(mix_machine.poke_memory(6u16, Operation::make_instruction(true, 0u16, 0u8, 2u8, 55u8)), Ok(())); // ENTX 0
    assert_eq!(mix_machine.poke_memory(7u16, Operation::make_instruction(true, 2u16, 0u8, 2u8, 47u8)), Ok(())); // JXP 2
    assert_eq!(mix_machine.poke_memory(8u16, Operation::make_instruction(true, 5u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 5

    assert_eq!(mix_machine.step(), Ok(())); // JXP 2 - should jump since X is positive
    assert_eq!(mix_machine.step(), Ok(())); // ENNA 20
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(20u32 + (1u32 << 30)));
    assert_eq!(mix_machine.step(), Ok(())); // ENNX 20 (i.e. -20)
    assert_eq!(mix_machine.step(), Ok(())); // JXP 6 - should not jump since X is negative
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 10
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(10u32));
    assert_eq!(mix_machine.step(), Ok(())); // ENTX 0
    assert_eq!(mix_machine.peek_register(Register::RegX), Ok(0u32));
    assert_eq!(mix_machine.step(), Ok(())); // JXP 2 - should not jump since X is zero
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 5
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(5u32));
}

#[test]
fn jump_i1_non_zero() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_register(Register::RegI1, 10u32), Ok(()));
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 2u16, 0u8, 4u8, 41u8)), Ok(())); // J1NZ 2
    assert_eq!(mix_machine.poke_memory(1u16, Operation::make_instruction(true, 10u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 10
    assert_eq!(mix_machine.poke_memory(2u16, Operation::make_instruction(true, 20u16, 0u8, 3u8, 48u8)), Ok(())); // ENNA 20
    assert_eq!(mix_machine.poke_memory(3u16, Operation::make_instruction(true, 20u16, 0u8, 3u8, 49u8)), Ok(())); // ENN1 20
    assert_eq!(mix_machine.poke_memory(4u16, Operation::make_instruction(true, 6u16, 0u8, 4u8, 41u8)), Ok(())); // J1NZ 6
    assert_eq!(mix_machine.poke_memory(5u16, Operation::make_instruction(true, 10u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 10
    assert_eq!(mix_machine.poke_memory(6u16, Operation::make_instruction(true, 0u16, 0u8, 2u8, 49u8)), Ok(())); // ENT1 0
    assert_eq!(mix_machine.poke_memory(7u16, Operation::make_instruction(true, 2u16, 0u8, 4u8, 41u8)), Ok(())); // J1NZ 2
    assert_eq!(mix_machine.poke_memory(8u16, Operation::make_instruction(true, 5u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 5

    assert_eq!(mix_machine.step(), Ok(())); // J1NZ 2 - should jump since I1 is positive
    assert_eq!(mix_machine.step(), Ok(())); // ENNA 20
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(20u32 + (1u32 << 30)));
    assert_eq!(mix_machine.step(), Ok(())); // ENN1 20 (i.e. -20)
    assert_eq!(mix_machine.step(), Ok(())); // J1NZ 6 - should jump since I1 is negative
    assert_eq!(mix_machine.step(), Ok(())); // ENT1 0
    assert_eq!(mix_machine.peek_register(Register::RegI1), Ok(0u32));
    assert_eq!(mix_machine.step(), Ok(())); // J1NZ 2 - should not jump since I1 is zero
    assert_eq!(mix_machine.step(), Ok(())); // ENTA 5
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(5u32));
}
