extern crate mix;

use mix::mix_machine::*;
use mix::mix_operations::*;

#[test]
fn enter_a() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 25u16, 0u8, 2u8, 48u8)), Ok(())); // ENTA 25, 0
    assert_eq!(mix_machine.poke_register(Register::RegA,  10u32), Ok(()));  // Make sure the actually gets overridden, not added to
    assert_eq!(mix_machine.poke_register(Register::RegI3, 5u32 + (1u32 << 30)), Ok(()));    // rI3 <- -5, just to test nothing happens
    assert_eq!(mix_machine.step(), Ok(()));

    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(25u32));
}

#[test]
fn enter_a_indexed() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 25u16, 3u8, 2u8, 48u8)), Ok(())); // ENTA 25, 3
    assert_eq!(mix_machine.poke_register(Register::RegA, 0u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegI3, 5u32 + (1u32 << 30)), Ok(()));    // rI3 <- -5
    assert_eq!(mix_machine.step(), Ok(()));

    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(20u32));
}

#[test]
fn enter_a_indexed_addr_non_zero() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 0u16, 3u8, 2u8, 48u8)), Ok(())); // ENTA +0, 3
    assert_eq!(mix_machine.poke_register(Register::RegA, 0u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegI3, 5u32 + (1u32 << 30)), Ok(()));    // rI32 <- -5
    assert_eq!(mix_machine.step(), Ok(()));

    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(5u32 + (1u32 << 30))); // rA = -5
}

#[test]
fn enter_a_indexed_addr_zero_pos() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 5u16, 3u8, 2u8, 48u8)), Ok(())); // ENTA +5, 3
    assert_eq!(mix_machine.poke_register(Register::RegA, 0u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegI3, 5u32 + (1u32 << 30)), Ok(()));    // rI32 <- -5
    assert_eq!(mix_machine.step(), Ok(()));

    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(0u32)); // rA = +0
}

#[test]
// Knuth, describing ENTA: If M=0, the sign of the instruction is loaded.
fn enter_a_indexed_addr_zero_neg() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(false, 5u16, 3u8, 2u8, 48u8)), Ok(())); // ENTA -5, 3
    assert_eq!(mix_machine.poke_register(Register::RegA, 0u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegI3, 5u32), Ok(()));    // rI32 <- 5
    assert_eq!(mix_machine.step(), Ok(()));

    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(1u32 << 30)); // rA = -0
}

#[test]
fn enter_a_indexed_addr_neg() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(false, 15u16, 3u8, 2u8, 48u8)), Ok(())); // ENTA -15, 3
    assert_eq!(mix_machine.poke_register(Register::RegA, 0u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegI3, 5u32), Ok(()));    // rI32 <- 5
    assert_eq!(mix_machine.step(), Ok(()));

    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(10u32 + (1u32 << 30))); // rA = -10
}

#[test]
fn enter_x_indexed_addr_neg() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(false, 15u16, 3u8, 2u8, 55u8)), Ok(())); // ENTX -15, 3
    assert_eq!(mix_machine.poke_register(Register::RegX, 0u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegI3, 5u32), Ok(()));    // rI32 <- 5
    assert_eq!(mix_machine.step(), Ok(()));

    assert_eq!(mix_machine.peek_register(Register::RegX), Ok(10u32 + (1u32 << 30))); // rX = -10
}

#[test]
fn enter_i1_indexed_addr_neg() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(false, 15u16, 3u8, 2u8, 49u8)), Ok(())); // ENTI1 -15, 3
    assert_eq!(mix_machine.poke_register(Register::RegI1, 0u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegI3, 5u32), Ok(()));    // rI32 <- 5
    assert_eq!(mix_machine.step(), Ok(()));

    assert_eq!(mix_machine.peek_register(Register::RegI1), Ok(10u32 + (1u32 << 30)));
}

#[test]
fn enter_i2_indexed_addr_neg() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(false, 15u16, 3u8, 2u8, 50u8)), Ok(())); // ENTI2 -15, 3
    assert_eq!(mix_machine.poke_register(Register::RegI2, 0u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegI3, 5u32), Ok(()));    // rI32 <- 5
    assert_eq!(mix_machine.step(), Ok(()));

    assert_eq!(mix_machine.peek_register(Register::RegI2), Ok(10u32 + (1u32 << 30)));
}

#[test]
fn enter_i3_indexed_addr_neg() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(false, 15u16, 6u8, 2u8, 51u8)), Ok(())); // ENTI3 -15, 6
    assert_eq!(mix_machine.poke_register(Register::RegI3, 0u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegI6, 5u32), Ok(()));    // rI6 <- 5
    assert_eq!(mix_machine.step(), Ok(()));

    assert_eq!(mix_machine.peek_register(Register::RegI3), Ok(10u32 + (1u32 << 30)));
}

#[test]
fn enter_i4_indexed_addr_neg() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(false, 15u16, 3u8, 2u8, 52u8)), Ok(())); // ENTI4 -15, 3
    assert_eq!(mix_machine.poke_register(Register::RegI4, 0u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegI3, 5u32), Ok(()));    // rI3 <- 5
    assert_eq!(mix_machine.step(), Ok(()));

    assert_eq!(mix_machine.peek_register(Register::RegI4), Ok(10u32 + (1u32 << 30)));
}

#[test]
fn enter_i5_indexed_addr_neg() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(false, 15u16, 3u8, 2u8, 53u8)), Ok(())); // ENTI5 -15, 3
    assert_eq!(mix_machine.poke_register(Register::RegI5, 0u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegI3, 5u32), Ok(()));    // rI3 <- 5
    assert_eq!(mix_machine.step(), Ok(()));

    assert_eq!(mix_machine.peek_register(Register::RegI5), Ok(10u32 + (1u32 << 30)));
}

#[test]
fn enter_i6_indexed_addr_neg() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(false, 15u16, 3u8, 2u8, 54u8)), Ok(())); // ENTI6 -15, 3
    assert_eq!(mix_machine.poke_register(Register::RegI6, 0u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegI3, 5u32), Ok(()));    // rI3 <- 5
    assert_eq!(mix_machine.step(), Ok(()));

    assert_eq!(mix_machine.peek_register(Register::RegI6), Ok(10u32 + (1u32 << 30)));
}

// The remaining tests aren't so extensive, since the switch statement identifying the target
// register has already been tested by the above.

#[test]
fn enter_negative_a_indexed() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 25u16, 3u8, 3u8, 48u8)), Ok(())); // ENNA 25, 3
    assert_eq!(mix_machine.poke_register(Register::RegA, 0u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegI3, 5u32 + (1u32 << 30)), Ok(()));    // rI3 <- -5
    assert_eq!(mix_machine.step(), Ok(()));

    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(20u32 + (1u32 << 30)));
}

#[test]
fn enter_negative_i1() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 25u16, 0u8, 3u8, 49u8)), Ok(())); // ENNI1 25
    assert_eq!(mix_machine.poke_register(Register::RegI1, 0u32), Ok(()));
    assert_eq!(mix_machine.step(), Ok(()));

    assert_eq!(mix_machine.peek_register(Register::RegI1), Ok(25u32 + (1u32 << 30)));
}

#[test]
fn inc_a_indexed() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 25u16, 3u8, 0u8, 48u8)), Ok(())); // INCA 25, 3
    assert_eq!(mix_machine.poke_register(Register::RegA, 10u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegI3, 5u32 + (1u32 << 30)), Ok(()));    // rI3 <- -5
    assert_eq!(mix_machine.step(), Ok(()));

    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(30u32)); // 10 + 20 = 30
}

#[test]
fn inc_4_indexed() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 25u16, 3u8, 0u8, 52u8)), Ok(())); // INC4 25, 3
    assert_eq!(mix_machine.poke_register(Register::RegI4, 10u32), Ok(()));
    assert_eq!(mix_machine.poke_register(Register::RegI3, 50u32 + (1u32 << 30)), Ok(()));    // rI3 <- -50
    assert_eq!(mix_machine.step(), Ok(()));

    assert_eq!(mix_machine.peek_register(Register::RegI4), Ok(15u32 + (1u32 << 30))); // 10 - 25 = -15
}

#[test]
fn dec_a() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 15u16, 0u8, 1u8, 48u8)), Ok(()));  // DECA 15, 0
    assert_eq!(mix_machine.poke_register(Register::RegA, 10u32), Ok(()));
    assert_eq!(mix_machine.step(), Ok(()));
    
    assert_eq!(mix_machine.peek_register(Register::RegA), Ok(5u32 + (1u32 << 30))); // -5
}

#[test]
//Knuth: Overflow in INCi results in undefined behaviour
// Our Ii registers can hold up to 2^12 - 1 = 4096
fn test_overflow_undefined() {
    let mut mix_machine = MixMachine::new();
    assert_eq!(mix_machine.poke_memory(0u16, Operation::make_instruction(true, 1500u16, 0u8, 0u8, 49u8)), Ok(()));  // INC1 1500, 0
    assert_eq!(mix_machine.poke_register(Register::RegI1, 3000u32), Ok(()));
    assert_eq!(mix_machine.step(), Err(MixMachineErr {message : String::from("Overflow on inc or dec resulted in undefined behaviour!")}));
}
