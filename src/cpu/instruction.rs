use super::Register;
use crate::util::*;

mod condition;
mod data;
mod shifter;
mod multiply;

pub use condition::ConditionCode;
pub use shifter::{ShiftOperand, ShiftOperation, ShifterOperand};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DecodeError {
    InvalidShifterOperand(u16),
}

type Result<T> = std::result::Result<T, DecodeError>;
