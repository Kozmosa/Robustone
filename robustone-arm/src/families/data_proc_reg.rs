//! Data Processing - Register: ADD reg, SUB, EOR, CSEL.

use robustone_core::{
    ir::{Operand, RegisterId},
    types::error::{DecodeErrorKind, DisasmError},
};

use crate::encoding;

/// Decode data-processing register instructions.
pub fn decode_data_processing_register(
    word: u32,
) -> Result<(&'static str, Vec<Operand>), DisasmError> {
    // ADD / SUB (shifted register): sf | op | S | 01011 | shift | N | Rm | imm6 | Rn | Rd
    if (word & 0x1F200000) == 0x0B000000 {
        let op = encoding::extract_op_bit30(word);
        let s = ((word >> 29) & 1) != 0;
        let rd = encoding::extract_rd(word);
        let rn = encoding::extract_rn(word);
        let rm = encoding::extract_rm(word);
        let _shift = encoding::extract_shift(word);
        let _imm6 = encoding::extract_imm6(word);

        let mnemonic = match (op, s) {
            (false, false) => "add",
            (false, true) => "adds",
            (true, false) => "sub",
            (true, true) => "subs",
        };

        let ops = vec![
            Operand::Register {
                register: aarch64_reg(rd),
            },
            Operand::Register {
                register: aarch64_reg(rn),
            },
            Operand::Register {
                register: aarch64_reg(rm),
            },
        ];

        return Ok((mnemonic, ops));
    }

    // Logical (shifted register): sf | opc(2) | 01010 | shift | N | Rm | imm6 | Rn | Rd
    // AND: opc=00, EOR: opc=10
    if (word & 0x1F200000) == 0x0A000000 {
        let opc = encoding::extract_opc(word);
        let rd = encoding::extract_rd(word);
        let rn = encoding::extract_rn(word);
        let rm = encoding::extract_rm(word);

        let mnemonic = match opc {
            0b00 => "and",
            0b10 => "eor",
            _ => {
                return Err(DisasmError::DecodeFailure {
                    kind: DecodeErrorKind::InvalidEncoding,
                    architecture: Some("aarch64".to_string()),
                    detail: format!("unrecognized logical opc: {opc}"),
                });
            }
        };

        let ops = vec![
            Operand::Register {
                register: aarch64_reg(rd),
            },
            Operand::Register {
                register: aarch64_reg(rn),
            },
            Operand::Register {
                register: aarch64_reg(rm),
            },
        ];

        return Ok((mnemonic, ops));
    }

    // Conditional select: sf | op | S | 11010 | op2(2) | Rm | cond | op3(1) | Rn | Rd
    // CSEL: op2=00 (but Capstone shows op2=1 for csel)
    // Actually in encoding: bits[30:29]=op, bit[28:24]=11010, bits[11:10]=op2
    if (word & 0x1F000000) == 0x1A000000 {
        let op2 = (word >> 10) & 0x3;
        if op2 == 0b00 {
            let rd = encoding::extract_rd(word);
            let rn = encoding::extract_rn(word);
            let rm = encoding::extract_rm(word);
            let cond = encoding::extract_cond(word);

            let ops = vec![
                Operand::Register {
                    register: aarch64_reg(rd),
                },
                Operand::Register {
                    register: aarch64_reg(rn),
                },
                Operand::Register {
                    register: aarch64_reg(rm),
                },
                Operand::Text {
                    value: condition_name(cond).to_string(),
                },
            ];

            return Ok(("csel", ops));
        }
    }

    Err(DisasmError::DecodeFailure {
        kind: DecodeErrorKind::InvalidEncoding,
        architecture: Some("aarch64".to_string()),
        detail: format!("unrecognized DP-reg encoding 0x{word:08x}"),
    })
}

fn condition_name(cond: u8) -> &'static str {
    match cond {
        0b0000 => "eq",
        0b0001 => "ne",
        0b0010 => "cs",
        0b0011 => "cc",
        0b0100 => "mi",
        0b0101 => "pl",
        0b0110 => "vs",
        0b0111 => "vc",
        0b1000 => "hi",
        0b1001 => "ls",
        0b1010 => "ge",
        0b1011 => "lt",
        0b1100 => "gt",
        0b1101 => "le",
        0b1110 => "al",
        _ => "??",
    }
}

fn aarch64_reg(id: u32) -> RegisterId {
    RegisterId {
        architecture: robustone_core::ir::ArchitectureId::Arm,
        id,
    }
}
