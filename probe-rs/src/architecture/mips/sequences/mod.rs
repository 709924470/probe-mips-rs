use std::sync::Arc;

use super::communication_interface::MipsCommunicationInterface;

pub mod pic32mz;

pub trait MipsDebugSequence: Send + Sync {
    fn on_connect(&self, _interface: &mut MipsCommunicationInterface) -> Result<(), crate::Error> {
        Ok(())
    }
}

pub struct DefaultMipsSequence(pub(crate) ());

impl DefaultMipsSequence {
    pub fn create() -> Arc<dyn MipsDebugSequence> {
        Arc::new(Self(()))
    }
}

impl MipsDebugSequence for DefaultMipsSequence {}
