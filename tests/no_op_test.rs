extern crate mix;
use mix::mix_machine::*;
use mix::mix_operations::*;

#[test]
// Just tests that the NOP runs
fn no_op() {
    let mut mix_machine = MixMachine::new();

    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 3u16, 2u8, 4u8, 0u8)), Ok(())); // NOP
    assert_eq!(mix_machine.step(), Ok(()));
}

