use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn prefetchwt1_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHWT1, operand1: Some(IndirectDisplaced(ESI, 726231803, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 13, 150, 251, 106, 73, 43], OperandSize::Dword)
}

#[test]
fn prefetchwt1_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHWT1, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 13, 22], OperandSize::Qword)
}

