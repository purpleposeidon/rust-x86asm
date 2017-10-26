use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub132ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 155, 209], OperandSize::Dword)
}

#[test]
fn vfmsub132ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 155, 7], OperandSize::Dword)
}

#[test]
fn vfmsub132ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 155, 208], OperandSize::Qword)
}

#[test]
fn vfmsub132ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 64627693, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 155, 188, 186, 237, 35, 218, 3], OperandSize::Qword)
}

#[test]
fn vfmsub132ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 220, 155, 244], OperandSize::Dword)
}

#[test]
fn vfmsub132ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 137, 155, 44, 250], OperandSize::Dword)
}

#[test]
fn vfmsub132ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 93, 145, 155, 203], OperandSize::Qword)
}

#[test]
fn vfmsub132ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 177458134, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 13, 138, 155, 164, 134, 214, 203, 147, 10], OperandSize::Qword)
}

