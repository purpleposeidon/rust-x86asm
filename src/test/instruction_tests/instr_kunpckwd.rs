use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kunpckwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KUNPCKWD, operand1: Some(Direct(K1)), operand2: Some(Direct(K1)), operand3: Some(Direct(K7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 75, 207], OperandSize::Dword)
}

#[test]
fn kunpckwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KUNPCKWD, operand1: Some(Direct(K5)), operand2: Some(Direct(K7)), operand3: Some(Direct(K1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 75, 233], OperandSize::Qword)
}

