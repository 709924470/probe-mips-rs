#![allow(unused, missing_docs)]
use crate::memory_mapped_bitfield_register;

pub mod communication_interface;
pub mod sequences;

pub mod assembly;
pub mod ejtag;
pub mod registers;

#[derive(Debug)]
pub struct MipsState {}

impl MipsState {
    pub(crate) fn new() -> Self {
        Self {}
    }
}