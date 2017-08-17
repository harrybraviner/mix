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

