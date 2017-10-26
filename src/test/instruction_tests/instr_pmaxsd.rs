use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 61, 216], OperandSize::Dword)
}

#[test]
fn pmaxsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 61, 41], OperandSize::Dword)
}

#[test]
fn pmaxsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 61, 192], OperandSize::Qword)
}

#[test]
fn pmaxsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 61, 22], OperandSize::Qword)
}

