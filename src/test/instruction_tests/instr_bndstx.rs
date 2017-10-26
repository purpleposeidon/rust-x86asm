use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bndstx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDSTX, operand1: Some(IndirectScaledIndexed(EAX, EDX, One, Some(OperandSize::Unsized), None)), operand2: Some(Direct(BND0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 27, 4, 16], OperandSize::Dword)
}

#[test]
fn bndstx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDSTX, operand1: Some(IndirectScaledIndexed(RDX, RAX, One, Some(OperandSize::Unsized), None)), operand2: Some(Direct(BND3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 27, 28, 2], OperandSize::Qword)
}

