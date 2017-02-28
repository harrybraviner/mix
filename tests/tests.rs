extern crate mix;

use mix::mix_machine::*;
use mix::mix_operations::*;

#[test]
fn test_load_op(){
    let mut mix_machine = MixMachine::new();
    mix_machine.poke_memory(0u16, Operation::make_instruction(true, 10u16, 0u8, 5u8, 8u8)); // LDA
    mix_machine.poke_memory(10u16, 1234u32); // value to load
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(0u32));
    mix_machine.step();
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(1234u32));
}

#[test]
fn test_load_op_with_offset(){
    let mut mix_machine = MixMachine::new();
    mix_machine.poke_memory(0u16, Operation::make_instruction(true, 3u16,  0u8, 5u8, 11u8)); // LDI3
    mix_machine.poke_memory(1u16, Operation::make_instruction(true, 10u16, 3u8, 5u8, 15u8)); // LDX
    mix_machine.poke_memory(3u16, 1000u32); // value to load into I3
    mix_machine.poke_memory(1010u16, ((1u32<<8) + 345u32)); // value to load into X
    assert_eq!(mix_machine.peek_register(Register::RegI3), Ok(0u32));
    assert_eq!(mix_machine.peek_register(Register::RegX),  Ok(0u32));
    mix_machine.step();
    assert_eq!(mix_machine.peek_register(Register::RegI3), Ok(1000u32));
    assert_eq!(mix_machine.peek_register(Register::RegX),  Ok(0u32));
    mix_machine.step();
    assert_eq!(mix_machine.peek_register(Register::RegI3), Ok(1000u32));
    assert_eq!(mix_machine.peek_register(Register::RegX),  Ok((1u32<<8) + 345u32));
}

#[test]
fn test_load_negative_op(){
    let mut mix_machine = MixMachine::new();
    mix_machine.poke_memory(0u16, Operation::make_instruction(true, 3u16,  0u8, 5u8, 19u8)); // LDI3
    mix_machine.poke_memory(1u16, Operation::make_instruction(true, 10u16, 0u8, 5u8, 23u8)); // LDXN
    mix_machine.poke_memory(3u16, 1000u32); // value to negate and then load into I3
    mix_machine.poke_memory(10u16, 0u32); // value to negate and then load into X
    assert_eq!(mix_machine.peek_register(Register::RegI3), Ok(0u32));
    assert_eq!(mix_machine.peek_register(Register::RegX),  Ok(0u32));
    mix_machine.step();
    mix_machine.step();
    assert_eq!(mix_machine.peek_register(Register::RegI3), Ok(1000u32 + (1u32 << 30)));
    assert_eq!(mix_machine.peek_register(Register::RegX),  Ok(0u32 + (1u32 << 30)));
}
