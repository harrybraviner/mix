extern crate mix;
use mix::mix_operations::*;
use mix::mix_machine::*;

#[test]
fn test_jmp() {
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
