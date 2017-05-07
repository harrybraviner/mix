extern crate mix;

use mix::mix_machine::*;
use mix::mix_operations::*;

#[test]
fn enter_a() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 25u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 25, 0
    assert_eq!(mix_machine.poke_register(Register::RegA, 0u32), Ok(()));
    assert_eq!(mix_machine.step(), Ok(()));

    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(25u32));
}
