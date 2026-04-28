//! AArch64 instruction text rendering.

use robustone_core::ir::{DecodedInstruction, Operand, TextRenderProfile};

/// Render an AArch64 decoded instruction into mnemonic and operand text.
pub fn render_aarch64_text_parts(
    instruction: &DecodedInstruction,
    _profile: TextRenderProfile,
    _alias_regs: bool,
    _capstone_aliases: bool,
    _compressed_aliases: bool,
    _unsigned_immediate: bool,
) -> (String, String) {
    let capstone_mnemonic = instruction
        .render_hints
        .capstone_mnemonic
        .as_ref()
        .unwrap_or(&instruction.mnemonic)
        .clone();

    let hidden: std::collections::HashSet<usize> = instruction
        .render_hints
        .capstone_hidden_operands
        .iter()
        .copied()
        .collect();

    let operands = instruction
        .operands
        .iter()
        .enumerate()
        .filter(|(idx, _)| !hidden.contains(idx))
        .map(|(idx, op)| format_aarch64_operand(instruction.mnemonic.as_str(), idx, op))
        .collect::<Vec<_>>()
        .join(", ");

    (capstone_mnemonic, operands)
}

fn format_aarch64_operand(mnemonic: &str, idx: usize, operand: &Operand) -> String {
    match operand {
        Operand::Register { register } => {
            // For CSEL, x31 in source positions (idx 1 or 2) is xzr, not sp
            if mnemonic == "csel" && (idx == 1 || idx == 2) && register.id == 31 {
                "xzr".to_string()
            } else {
                aarch64_register_name(register.id)
            }
        }
        Operand::Immediate { value } => {
            // Branch targets are printed without # prefix in Capstone
            if mnemonic == "b" || mnemonic == "bl" {
                format!("{value}")
            } else if *value >= 0 && *value < 10 {
                format!("#{value}")
            } else {
                format!("#0x{value:x}")
            }
        }
        Operand::Text { value } => value.clone(),
        Operand::Memory { base, displacement } => {
            if let Some(base) = base {
                format!("[{}, #{}]", aarch64_register_name(base.id), displacement)
            } else {
                format!("[#{displacement}]")
            }
        }
    }
}

fn aarch64_register_name(id: u32) -> String {
    match id {
        0..=30 => format!("x{id}"),
        31 => "sp".to_string(),
        _ => format!("r{id}"),
    }
}
