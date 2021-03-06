use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn knotd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KNOTD, operand1: Some(Direct(K6)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 68, 244], OperandSize::Dword)
}

fn knotd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KNOTD, operand1: Some(Direct(K7)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 68, 250], OperandSize::Qword)
}

