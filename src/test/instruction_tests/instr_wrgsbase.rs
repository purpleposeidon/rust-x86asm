use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn wrgsbase_1() {
    run_test(&Instruction { mnemonic: Mnemonic::WRGSBASE, operand1: Some(Direct(EBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 174, 219], OperandSize::Qword)
}

#[test]
fn wrgsbase_2() {
    run_test(&Instruction { mnemonic: Mnemonic::WRGSBASE, operand1: Some(Direct(RSI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 174, 222], OperandSize::Qword)
}

