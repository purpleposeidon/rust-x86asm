use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn maskmovq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MASKMOVQ, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 247, 204], OperandSize::Word)
}

#[test]
fn maskmovq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MASKMOVQ, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 247, 253], OperandSize::Dword)
}

#[test]
fn maskmovq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MASKMOVQ, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 247, 221], OperandSize::Qword)
}

