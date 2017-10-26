use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rdrand_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RDRAND, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 199, 242], OperandSize::Dword)
}

#[test]
fn rdrand_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RDRAND, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 199, 246], OperandSize::Qword)
}

#[test]
fn rdrand_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RDRAND, operand1: Some(Direct(EDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 242], OperandSize::Dword)
}

#[test]
fn rdrand_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RDRAND, operand1: Some(Direct(ECX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 241], OperandSize::Qword)
}

#[test]
fn rdrand_5() {
    run_test(&Instruction { mnemonic: Mnemonic::RDRAND, operand1: Some(Direct(RSI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 199, 246], OperandSize::Qword)
}

