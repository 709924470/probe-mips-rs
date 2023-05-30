use crate::architecture::mips::sequences::MipsDebugSequence;
use std::sync::Arc;

pub struct PIC32MZ(());

impl PIC32MZ {
    pub fn create() -> Arc<dyn MipsDebugSequence> {
        Arc::new(Self(()))
    }
}

impl MipsDebugSequence for PIC32MZ {
    fn on_connect(
        &self,
        _interface: &mut crate::architecture::mips::communication_interface::MIPSCommunicationInterface,
    ) -> Result<(), crate::Error> {
        tracing::info!("Disabling Memory write protect on pic32mz...");

        // TODO: !("Write 0xaa996655 & 0x556699aa to SYSKEY");

        Ok(())
    }
}
