use mix_machine;

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
    register: Register,
    field: u8,
    negative: bool,
}

impl Operation {
    pub fn from_u32(instruction: u32) -> Result<Operation, ()> {
        let op_code: u8    = ( instruction        % 64u32) as u8;
        let field_spec: u8 = ((instruction >> 6 ) % 64u32) as u8;
        let index_spec: u8 = ((instruction >> 12) % 64u32) as u8;
        let address: u16   = ((instruction >> 18) % 4096u32) as u16;
        let sign: i8       = if (instruction >> 30) % 2 == 1 { -1i8 } else { 1i8 };
        
        match op_code {
            // Load instructions
            8  => Ok(Load(LoadOp {register: RegA,  field: field_spec, negative: false})),
            15 => Ok(Load(LoadOp {register: RegX,  field: field_spec, negative: false})),
            9  => Ok(Load(LoadOp {register: RegI1, field: field_spec, negative: false})),
            10 => Ok(Load(LoadOp {register: RegI2, field: field_spec, negative: false})),
            11 => Ok(Load(LoadOp {register: RegI3, field: field_spec, negative: false})),
            12 => Ok(Load(LoadOp {register: RegI4, field: field_spec, negative: false})),
            13 => Ok(Load(LoadOp {register: RegI5, field: field_spec, negative: false})),
            14 => Ok(Load(LoadOp {register: RegI6, field: field_spec, negative: false})),
            // Ok(Load negative instructions
            16 => Ok(Load(LoadOp {register: RegA,  field: field_spec, negative: true})),
            23 => Ok(Load(LoadOp {register: RegX,  field: field_spec, negative: true})),
            17 => Ok(Load(LoadOp {register: RegI1, field: field_spec, negative: true})),
            18 => Ok(Load(LoadOp {register: RegI2, field: field_spec, negative: true})),
            19 => Ok(Load(LoadOp {register: RegI3, field: field_spec, negative: true})),
            20 => Ok(Load(LoadOp {register: RegI4, field: field_spec, negative: true})),
            21 => Ok(Load(LoadOp {register: RegI5, field: field_spec, negative: true})),
            22 => Ok(Load(LoadOp {register: RegI6, field: field_spec, negative: true})),
            // Unknown (or not implemented)
            _  => Err(())
        }
    }
}


pub enum StoreOp { STA, STX, ST1, ST2, ST3, ST4, ST5, ST6, STJ, STZ }

pub enum ArithOp { ADD, SUB, MUL, DIV }

pub enum AddressOp { ENTA, ENTX, ENT1, ENT2, ENT3, ENT4, ENT5, ENT6, 
                 ENNA, ENNX, ENN1, ENN2, ENN3, ENN4, ENN5, ENN6,
                 INCA, INCX, INC1, INC2, INC3, INC4, INC5, INC6,
                 DECA, DECX, DEC1, DEC2, DEC3, DEC4, DEC5, DEC6 }

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
    
