extern crate mix;
use mix::mix_machine::*;
use mix::mix_operations::*;

#[test]
fn move_single_word() {
    let mut mix_machine = MixMachine::new();
    
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 1000u16, 0u8, 1u8, 7u8)), Ok(())); // MOVE 1000, (1)
    assert_eq!(mix_machine.poke_register(Register::RegI1, 1005u32), Ok(()));    // Specifies the destination for the move
    assert_eq!(mix_machine.poke_memory(1000u16, 20u32), Ok(())); // MOVE 1000, (1)

    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_memory(1000u16), Ok(20u32));
    assert_eq!(mix_machine.peek_memory(1005u16), Ok(20u32));
    assert_eq!(mix_machine.peek_register(Register::RegI1), Ok(1006u32));
}

#[test]
fn move_two_words() {
    let mut mix_machine = MixMachine::new();
    
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 1000u16, 0u8, 2u8, 7u8)), Ok(())); // MOVE 1000, (1)
    assert_eq!(mix_machine.poke_register(Register::RegI1, 1005u32), Ok(()));    // Specifies the destination for the move
    assert_eq!(mix_machine.poke_memory(1000u16, 20u32), Ok(()));
    assert_eq!(mix_machine.poke_memory(1001u16, 25u32), Ok(()));

    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_memory(1000u16), Ok(20u32));
    assert_eq!(mix_machine.peek_memory(1001u16), Ok(25u32));
    assert_eq!(mix_machine.peek_memory(1005u16), Ok(20u32));
    assert_eq!(mix_machine.peek_memory(1006u16), Ok(25u32));
    assert_eq!(mix_machine.peek_register(Register::RegI1), Ok(1007u32));
}


#[test]
fn move_overlapping() {
    let mut mix_machine = MixMachine::new();
    
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 1000u16, 0u8, 2u8, 7u8)), Ok(())); // MOVE 1000, (1)
    assert_eq!(mix_machine.poke_register(Register::RegI1, 1001u32), Ok(()));    // Specifies the destination for the move
    assert_eq!(mix_machine.poke_memory(1000u16, 20u32), Ok(()));
    assert_eq!(mix_machine.poke_memory(1001u16, 25u32), Ok(()));

    assert_eq!(mix_machine.step(), Ok(()));
    assert_eq!(mix_machine.peek_memory(1000u16), Ok(20u32));
    assert_eq!(mix_machine.peek_memory(1001u16), Ok(20u32));
    assert_eq!(mix_machine.peek_memory(1002u16), Ok(20u32));
    assert_eq!(mix_machine.peek_register(Register::RegI1), Ok(1003u32));
}
