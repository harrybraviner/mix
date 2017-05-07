pub enum Operation {
    Load(LoadOp),
    Store(StoreOp),
    Arithmetic(ArithOp),
    AddressTransfer(AddressOp),
    Comparison(CompOp),
    Jump(JumpOp),
    Unknown,
}

use mix_operations::Operation::*;
use mix_machine::Register;
use mix_machine::Register::*;

pub struct LoadOp {
    pub register: Register,
    pub field: u8,
    pub negative: bool,
    pub address: i16,   // Note that the address of the op may be negative, and -0 and +0 do the same thing
    pub index_spec: u8,
}

pub struct StoreOp {
    pub register: Option<Register>, // None causes a zero to be stored
    pub field: u8,
    pub address: i16,   // Note that the address of the op may be negative, and -0 and +0 do the same thing
    pub index_spec: u8,
}

pub enum ArithOpType {
    Addition, Subtraction, Multiplication, Division,
}

pub struct ArithOp {
    pub op_type: ArithOpType,
    pub field: u8,
    pub address: i16,
    pub index_spec: u8,
}

pub struct AddressOp {
    pub register: Register,
    pub address: i16,
    pub negative_address: bool,
    pub index_spec: u8,
    pub increase: bool  // 'Increase' as opposed to 'enter'.
}

impl Operation {
    pub fn from_u32(instruction: u32) -> Result<Operation, ()> {
        let op_code: u8    = ( instruction        % 64u32) as u8;
        let field_spec: u8 = ((instruction >> 6 ) % 64u32) as u8;
        let index_spec: u8 = ((instruction >> 12) % 64u32) as u8;
        let negative_address: bool = instruction & (1u32 << 30) != 0;
        let address: i16   = ((instruction >> 18) % 4096u32) as i16 * (if negative_address { -1i16 } else { 1i16 });
        
        match op_code {
            // Load instructions
            8  => Ok(Load(LoadOp {register: RegA,  field: field_spec, negative: false, address: address, index_spec: index_spec})),
            15 => Ok(Load(LoadOp {register: RegX,  field: field_spec, negative: false, address: address, index_spec: index_spec})),
            9  => Ok(Load(LoadOp {register: RegI1, field: field_spec, negative: false, address: address, index_spec: index_spec})),
            10 => Ok(Load(LoadOp {register: RegI2, field: field_spec, negative: false, address: address, index_spec: index_spec})),
            11 => Ok(Load(LoadOp {register: RegI3, field: field_spec, negative: false, address: address, index_spec: index_spec})),
            12 => Ok(Load(LoadOp {register: RegI4, field: field_spec, negative: false, address: address, index_spec: index_spec})),
            13 => Ok(Load(LoadOp {register: RegI5, field: field_spec, negative: false, address: address, index_spec: index_spec})),
            14 => Ok(Load(LoadOp {register: RegI6, field: field_spec, negative: false, address: address, index_spec: index_spec})),
            // Load negative instructions
            16 => Ok(Load(LoadOp {register: RegA,  field: field_spec, negative: true, address: address, index_spec: index_spec})),
            23 => Ok(Load(LoadOp {register: RegX,  field: field_spec, negative: true, address: address, index_spec: index_spec})),
            17 => Ok(Load(LoadOp {register: RegI1, field: field_spec, negative: true, address: address, index_spec: index_spec})),
            18 => Ok(Load(LoadOp {register: RegI2, field: field_spec, negative: true, address: address, index_spec: index_spec})),
            19 => Ok(Load(LoadOp {register: RegI3, field: field_spec, negative: true, address: address, index_spec: index_spec})),
            20 => Ok(Load(LoadOp {register: RegI4, field: field_spec, negative: true, address: address, index_spec: index_spec})),
            21 => Ok(Load(LoadOp {register: RegI5, field: field_spec, negative: true, address: address, index_spec: index_spec})),
            22 => Ok(Load(LoadOp {register: RegI6, field: field_spec, negative: true, address: address, index_spec: index_spec})),
            // Store instructions
            24 => Ok(Store(StoreOp {register: Some(RegA),  field: field_spec, address: address, index_spec: index_spec})),
            31 => Ok(Store(StoreOp {register: Some(RegX),  field: field_spec, address: address, index_spec: index_spec})),
            25 => Ok(Store(StoreOp {register: Some(RegI1), field: field_spec, address: address, index_spec: index_spec})),
            26 => Ok(Store(StoreOp {register: Some(RegI2), field: field_spec, address: address, index_spec: index_spec})),
            27 => Ok(Store(StoreOp {register: Some(RegI3), field: field_spec, address: address, index_spec: index_spec})),
            28 => Ok(Store(StoreOp {register: Some(RegI4), field: field_spec, address: address, index_spec: index_spec})),
            29 => Ok(Store(StoreOp {register: Some(RegI5), field: field_spec, address: address, index_spec: index_spec})),
            30 => Ok(Store(StoreOp {register: Some(RegI6), field: field_spec, address: address, index_spec: index_spec})),
            32 => Ok(Store(StoreOp {register: Some(RegJ),  field: field_spec, address: address, index_spec: index_spec})),
            33 => Ok(Store(StoreOp {register: None,        field: field_spec, address: address, index_spec: index_spec})),    // STZ, stores zero
            // Arithmetic instructions
            1  => Ok(Arithmetic(ArithOp {op_type: ArithOpType::Addition,       field: field_spec, address: address, index_spec: index_spec })),
            2  => Ok(Arithmetic(ArithOp {op_type: ArithOpType::Subtraction,    field: field_spec, address: address, index_spec: index_spec })),
            3  => Ok(Arithmetic(ArithOp {op_type: ArithOpType::Multiplication, field: field_spec, address: address, index_spec: index_spec })),
            4  => Ok(Arithmetic(ArithOp {op_type: ArithOpType::Division,       field: field_spec, address: address, index_spec: index_spec })),
            // Address transfer instructions
            48 => Ok(AddressTransfer(AddressOp {register: RegA, address: address, negative_address: negative_address, index_spec: index_spec, increase: false})),

            // Unknown (or not implemented)
            _  => Err(())
        }
    }

    pub fn make_instruction(positive: bool, address: u16, index_spec: u8, field_spec: u8, op_code: u8) -> u32 {
        if address >= (1u16 << 12) { panic!("Invalid address.") }
        if index_spec >= (1u8 << 6) { panic!("Invalid index specification.") }
        if field_spec >= (1u8 << 6) { panic!("Invalid field specification.") }
        if op_code >= (1u8 << 6) { panic!("Invalid op code.") }
        let sgn_bit = if positive { 0u32 } else { 1u32 << 30 };
        sgn_bit + ((address as u32) << 18) + ((index_spec as u32) << 12) + ((field_spec as u32) << 6) + (op_code as u32)
    }
}

pub enum CompOp { CMPA, CMPX, CMP1, CMP2, CMP3, CMP4, CMP5, CMP6 }

pub enum JumpOp { JMP, JSJ, JOV, JNOV, JL, JE, JG, JGE, JNE, JLE,
              JAN, JAZ, JAP, JANN, JANZ, JANP,
              JXN, JXZ, JXP, JXNN, JXNZ, JXNP,
              J1N, J1Z, J1P, J1NN, J1NZ, J1NP,
              J2N, J2Z, J2P, J2NN, J2NZ, J2NP,
              J3N, J3Z, J3P, J3NN, J3NZ, J3NP,
              J4N, J4Z, J4P, J4NN, J4NZ, J4NP,
              J5N, J5Z, J5P, J5NN, J5NZ, J5NP,
              J6N, J6Z, J6P, J6NN, J6NZ, J6NP }

              // FIXME - still need to add the Miscellaneous operators
    
