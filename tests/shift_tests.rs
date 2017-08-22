extern crate mix;

use mix::mix_machine::*;
use mix::mix_operations::*;

#[test]
fn shift_left_a() {
    let mut mix_machine = MixMachine::new();

    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 1u16, 2u8, 0u8, 6u8)), Ok(())); // SLA 1, 2
    assert_eq!(mix_machine.poke_register(Register::RegI2, 2u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegA, ((1u32 << 24) + (2u32 << 18) + (3u32 << 12) + (4u32 << 6) + 5u32)), Ok(()));

    assert_eq!(mix_machine.step(), Ok(())); // Should shift left (1 + rI2) = 3 bytes.
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok((4u32 << 24) + (5u32 << 18)));
}

#[test]
// Check that the sign bit is unaffected
fn shift_left_a_negative() {
    let mut mix_machine = MixMachine::new();

    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 1u16, 2u8, 0u8, 6u8)), Ok(())); // SLA 1, 2
    assert_eq!(mix_machine.poke_register(Register::RegI2, 2u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegA, ((1u32 << 30) + (1u32 << 24) + (2u32 << 18) + (3u32 << 12) + (4u32 << 6) + 5u32)), Ok(()));

    assert_eq!(mix_machine.step(), Ok(())); // Should shift left (1 + rI2) = 3 bytes.
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok((1u32 << 30) + (4u32 << 24) + (5u32 << 18)));
}

#[test]
fn shift_right_a() {
    let mut mix_machine = MixMachine::new();

    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 1u16, 2u8, 1u8, 6u8)), Ok(())); // SRA 1, 2
    assert_eq!(mix_machine.poke_register(Register::RegI2, 2u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegA, ((1u32 << 24) + (2u32 << 18) + (3u32 << 12) + (4u32 << 6) + 5u32)), Ok(()));

    assert_eq!(mix_machine.step(), Ok(())); // Should shift left (1 + rI2) = 3 bytes.
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok((1u32 << 6) + 2u32));
}

#[test]
fn shift_right_a_negative() {
    let mut mix_machine = MixMachine::new();

    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 1u16, 2u8, 1u8, 6u8)), Ok(())); // SRA 1, 2
    assert_eq!(mix_machine.poke_register(Register::RegI2, 2u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegA, ((1u32 << 30) + (1u32 << 24) + (2u32 << 18) + (3u32 << 12) + (4u32 << 6) + 5u32)), Ok(()));

    assert_eq!(mix_machine.step(), Ok(())); // Should shift left (1 + rI2) = 3 bytes.
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok((1u32 << 30) + (1u32 << 6) + 2u32));
}

#[test]
fn shift_left_ax() {
    let mut mix_machine = MixMachine::new();

    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 1u16, 2u8, 2u8, 6u8)), Ok(())); // SLAX 1, 2
    assert_eq!(mix_machine.poke_register(Register::RegI2, 2u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegA, ((1u32 << 24) + (2u32 << 18) + (3u32 << 12) + (4u32 << 6) + 5u32)), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegX, ((6u32 << 24) + (7u32 << 18) + (8u32 << 12) + (9u32 << 6) + 10u32)), Ok(()));

    assert_eq!(mix_machine.step(), Ok(())); // Should shift left (1 + rI2) = 3 bytes.
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok((4u32 << 24) + (5u32 << 18) + (6u32 << 12) + (7u32 << 6) + 8u32));
    assert_eq!(mix_machine.peek_register(Register::RegX), Ok((9u32 << 24) + (10u32 << 18)));
}

#[test]
fn shift_left_ax_large() {
    let mut mix_machine = MixMachine::new();

    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 6u16, 2u8, 2u8, 6u8)), Ok(())); // SLAX 6, 2
    assert_eq!(mix_machine.poke_register(Register::RegI2, 2u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegA, ((1u32 << 24) + (2u32 << 18) + (3u32 << 12) + (4u32 << 6) + 5u32)), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegX, ((6u32 << 24) + (7u32 << 18) + (8u32 << 12) + (9u32 << 6) + 10u32)), Ok(()));

    assert_eq!(mix_machine.step(), Ok(())); // Should shift left (6 + rI2) = 8 bytes.
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok((9u32 << 24) + (10u32 << 18)));
    assert_eq!(mix_machine.peek_register(Register::RegX), Ok(0u32));
}

#[test]
fn shift_left_ax_negative() {
    let mut mix_machine = MixMachine::new();

    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 1u16, 2u8, 2u8, 6u8)), Ok(())); // SLAX 1, 2
    assert_eq!(mix_machine.poke_register(Register::RegI2, 2u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegA, ((1u32 << 30) + (1u32 << 24) + (2u32 << 18) + (3u32 << 12) + (4u32 << 6) + 5u32)), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegX, ((1u32 << 30) + (6u32 << 24) + (7u32 << 18) + (8u32 << 12) + (9u32 << 6) + 10u32)), Ok(()));

    assert_eq!(mix_machine.step(), Ok(())); // Should shift left (1 + rI2) = 3 bytes.
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok((1u32 << 30) + (4u32 << 24) + (5u32 << 18) + (6u32 << 12) + (7u32 << 6) + 8u32));
    assert_eq!(mix_machine.peek_register(Register::RegX), Ok((1u32 << 30) + (9u32 << 24) + (10u32 << 18)));
}

#[test]
fn shift_left_ax_circulating() {
    let mut mix_machine = MixMachine::new();

    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 1u16, 2u8, 4u8, 6u8)), Ok(())); // SLC 1, 2
    assert_eq!(mix_machine.poke_register(Register::RegI2, 2u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegA, ((1u32 << 24) + (2u32 << 18) + (3u32 << 12) + (4u32 << 6) + 5u32)), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegX, ((6u32 << 24) + (7u32 << 18) + (8u32 << 12) + (9u32 << 6) + 10u32)), Ok(()));

    assert_eq!(mix_machine.step(), Ok(())); // Should shift left (1 + rI2) = 3 bytes.
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok((4u32 << 24) + (5u32  << 18) + (6u32 << 12) + (7u32 << 6) + 8u32));
    assert_eq!(mix_machine.peek_register(Register::RegX), Ok((9u32 << 24) + (10u32 << 18) + (1u32 << 12) + (2u32 << 6) + 3u32));
}

#[test]
fn shift_right_ax() {
    let mut mix_machine = MixMachine::new();

    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 1u16, 2u8, 3u8, 6u8)), Ok(())); // SRAX 1, 2
    assert_eq!(mix_machine.poke_register(Register::RegI2, 2u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegA, ((1u32 << 24) + (2u32 << 18) + (3u32 << 12) + (4u32 << 6) + 5u32)), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegX, ((6u32 << 24) + (7u32 << 18) + (8u32 << 12) + (9u32 << 6) + 10u32)), Ok(()));

    assert_eq!(mix_machine.step(), Ok(())); // Should shift left (1 + rI2) = 3 bytes.
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok((1u32 << 6) + 2u32));
    assert_eq!(mix_machine.peek_register(Register::RegX), Ok((3u32 << 24) + (4u32 << 18) + (5u32 << 12) + (6u32 << 6) + 7u32));
}

#[test]
fn shift_right_ax_large() {
    let mut mix_machine = MixMachine::new();

    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 6u16, 2u8, 3u8, 6u8)), Ok(())); // SRAX 6, 2
    assert_eq!(mix_machine.poke_register(Register::RegI2, 2u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegA, ((1u32 << 24) + (2u32 << 18) + (3u32 << 12) + (4u32 << 6) + 5u32)), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegX, ((6u32 << 24) + (7u32 << 18) + (8u32 << 12) + (9u32 << 6) + 10u32)), Ok(()));

    assert_eq!(mix_machine.step(), Ok(())); // Should shift left (1 + rI2) = 8 bytes.
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(0u32));
    assert_eq!(mix_machine.peek_register(Register::RegX), Ok((1u32 << 6) + 2u32));
}

#[test]
fn shift_right_ax_circulating() {
    let mut mix_machine = MixMachine::new();

    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 1u16, 2u8, 5u8, 6u8)), Ok(())); // SRC 1, 2
    assert_eq!(mix_machine.poke_register(Register::RegI2, 2u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegA, ((1u32 << 24) + (2u32 << 18) + (3u32 << 12) + (4u32 << 6) + 5u32)), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegX, ((6u32 << 24) + (7u32 << 18) + (8u32 << 12) + (9u32 << 6) + 10u32)), Ok(()));

    assert_eq!(mix_machine.step(), Ok(())); // Should shift left (1 + rI2) = 3 bytes.
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok((8u32 << 24) + (9u32 << 18) + (10u32 << 12) + (1u32 << 6) + 2u32));
    assert_eq!(mix_machine.peek_register(Register::RegX), Ok((3u32 << 24) + (4u32 << 18) + (5u32  << 12) + (6u32 << 6) + 7u32));
}

#[test]
fn shift_left_ax_circulating_large() {
    let mut mix_machine = MixMachine::new();

    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 6u16, 2u8, 4u8, 6u8)), Ok(())); // SLC 6, 2
    assert_eq!(mix_machine.poke_register(Register::RegI2, 2u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegA, ((1u32 << 24) + (2u32 << 18) + (3u32 << 12) + (4u32 << 6) + 5u32 )), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegX, ((6u32 << 24) + (7u32 << 18) + (8u32 << 12) + (9u32 << 6) + 10u32)), Ok(()));

    assert_eq!(mix_machine.step(), Ok(())); // Should shift left (1 + rI2) = 8 bytes.
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok((9u32 << 24) + (10u32 << 18) + (1u32 << 12) + (2u32 << 6) + 3u32));
    assert_eq!(mix_machine.peek_register(Register::RegX), Ok((4u32 << 24) + (5u32  << 18) + (6u32 << 12) + (7u32 << 6) + 8u32));
}

#[test]
fn shift_right_ax_circulating_large() {
    let mut mix_machine = MixMachine::new();

    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 6u16, 2u8, 5u8, 6u8)), Ok(())); // SRAX 6, 2
    assert_eq!(mix_machine.poke_register(Register::RegI2, 2u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegA, ((1u32 << 24) + (2u32 << 18) + (3u32 << 12) + (4u32 << 6) + 5u32 )), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegX, ((6u32 << 24) + (7u32 << 18) + (8u32 << 12) + (9u32 << 6) + 10u32)), Ok(()));

    assert_eq!(mix_machine.step(), Ok(())); // Should shift right (1 + rI2) = 8 bytes.
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok((3u32 << 24) + (4u32 << 18) + (5u32  << 12) + (6u32 << 6) + 7u32));
    assert_eq!(mix_machine.peek_register(Register::RegX), Ok((8u32 << 24) + (9u32 << 18) + (10u32 << 12) + (1u32 << 6) + 2u32));
}

#[test]
fn shift_right_ax_circulating_five() {
    let mut mix_machine = MixMachine::new();

    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 3u16, 2u8, 5u8, 6u8)), Ok(())); // SRAX 3, 2
    assert_eq!(mix_machine.poke_register(Register::RegI2, 2u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegA, ((1u32 << 24) + (2u32 << 18) + (3u32 << 12) + (4u32 << 6) + 5u32 )), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegX, ((6u32 << 24) + (7u32 << 18) + (8u32 << 12) + (9u32 << 6) + 10u32)), Ok(()));

    assert_eq!(mix_machine.step(), Ok(())); // Should shift right (1 + rI2) = 5 bytes.
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok((6u32 << 24) + (7u32 << 18) + (8u32 << 12) + (9u32 << 6) + 10u32));
    assert_eq!(mix_machine.peek_register(Register::RegX), Ok((1u32 << 24) + (2u32 << 18) + (3u32 << 12) + (4u32 << 6) + 5u32 ));
}

#[test]
fn shift_right_ax_circulating_ten() {
    let mut mix_machine = MixMachine::new();

    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 8u16, 2u8, 5u8, 6u8)), Ok(())); // SRAX 8, 2
    assert_eq!(mix_machine.poke_register(Register::RegI2, 2u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegA, ((1u32 << 24) + (2u32 << 18) + (3u32 << 12) + (4u32 << 6) + 5u32 )), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegX, ((6u32 << 24) + (7u32 << 18) + (8u32 << 12) + (9u32 << 6) + 10u32)), Ok(()));

    assert_eq!(mix_machine.step(), Ok(())); // Should shift right (1 + rI2) = 10 bytes.
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok((1u32 << 24) + (2u32 << 18) + (3u32 << 12) + (4u32 << 6) + 5u32 ));
    assert_eq!(mix_machine.peek_register(Register::RegX), Ok((6u32 << 24) + (7u32 << 18) + (8u32 << 12) + (9u32 << 6) + 10u32));
}

#[test]
fn shift_left_ax_circulating_five() {
    let mut mix_machine = MixMachine::new();

    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 3u16, 2u8, 4u8, 6u8)), Ok(())); // SLAX 3, 2
    assert_eq!(mix_machine.poke_register(Register::RegI2, 2u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegA, ((1u32 << 24) + (2u32 << 18) + (3u32 << 12) + (4u32 << 6) + 5u32 )), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegX, ((6u32 << 24) + (7u32 << 18) + (8u32 << 12) + (9u32 << 6) + 10u32)), Ok(()));

    assert_eq!(mix_machine.step(), Ok(())); // Should shift right (1 + rI2) = 5 bytes.
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok((6u32 << 24) + (7u32 << 18) + (8u32 << 12) + (9u32 << 6) + 10u32));
    assert_eq!(mix_machine.peek_register(Register::RegX), Ok((1u32 << 24) + (2u32 << 18) + (3u32 << 12) + (4u32 << 6) + 5u32 ));
}

#[test]
fn shift_left_ax_circulating_ten() {
    let mut mix_machine = MixMachine::new();

    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 8u16, 2u8, 4u8, 6u8)), Ok(())); // SLAX 8, 2
    assert_eq!(mix_machine.poke_register(Register::RegI2, 2u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegA, ((1u32 << 24) + (2u32 << 18) + (3u32 << 12) + (4u32 << 6) + 5u32 )), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegX, ((6u32 << 24) + (7u32 << 18) + (8u32 << 12) + (9u32 << 6) + 10u32)), Ok(()));

    assert_eq!(mix_machine.step(), Ok(())); // Should shift right (1 + rI2) = 10 bytes.
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok((1u32 << 24) + (2u32 << 18) + (3u32 << 12) + (4u32 << 6) + 5u32 ));
    assert_eq!(mix_machine.peek_register(Register::RegX), Ok((6u32 << 24) + (7u32 << 18) + (8u32 << 12) + (9u32 << 6) + 10u32));
}
