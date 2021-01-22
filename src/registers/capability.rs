//! Host Controller Capability Registers

use bit_field::BitField;
use core::convert::TryInto;

/// Capability Registers Length
#[repr(transparent)]
#[allow(clippy::module_name_repetitions)]
pub struct CapabilityRegistersLength(u8);
impl CapabilityRegistersLength {
    /// Returns the length of the Capability Registers.
    #[must_use]
    pub fn get(&self) -> u8 {
        self.0
    }
}

/// Structural Parameters 1
#[repr(transparent)]
pub struct StructuralParameters1(u32);
impl StructuralParameters1 {
    /// Returns the number of available device slots.
    #[must_use]
    pub fn number_of_device_slots(&self) -> u8 {
        self.0.get_bits(0..=7).try_into().unwrap()
    }

    /// Returns the number of ports.
    #[must_use]
    pub fn number_of_ports(&self) -> u8 {
        self.0.get_bits(24..=31).try_into().unwrap()
    }
}

/// Structural Parameters 2
#[repr(transparent)]
pub struct StructuralParameters2(u32);
impl StructuralParameters2 {
    /// Returns the maximum number of the elements the Event Ring Segment Table can contain.
    ///
    /// Note that the `ERST Max` field of the Structural Parameters 2 register contains the exponential
    /// value, but this method returns the calculated value.
    #[must_use]
    pub fn event_ring_segment_table_max(&self) -> u16 {
        2_u16.pow(self.erst_max())
    }

    /// Returns the number of scratchpads that xHC needs.
    #[must_use]
    pub fn max_scratchpad_buffers(&self) -> u32 {
        let h = self.max_scratchpad_buffers_hi();
        let l = self.max_scratchpad_buffers_lo();

        h << 5 | l
    }

    fn erst_max(&self) -> u32 {
        self.0.get_bits(4..=7)
    }

    fn max_scratchpad_buffers_hi(&self) -> u32 {
        self.0.get_bits(20..=25)
    }

    fn max_scratchpad_buffers_lo(&self) -> u32 {
        self.0.get_bits(27..=31)
    }
}

/// Capability Parameters 1
#[repr(transparent)]
#[allow(clippy::module_name_repetitions)]
pub struct CapabilityParameters1(u32);
impl CapabilityParameters1 {
    /// Returns `true` if the xHC uses 64 byte Context data structures, and `false` if the xHC uses
    /// 32 byte Context data structures.
    #[must_use]
    pub fn context_size(&self) -> bool {
        self.0.get_bit(2)
    }

    /// Returns the offset of the xHCI extended capability list from the MMIO base. If this value is
    /// zero, the list does not exist.
    /// The base address can be calculated by `(MMIO base) + (xECP) << 2`
    #[must_use]
    pub fn xhci_extended_capabilities_pointer(&self) -> u16 {
        self.0.get_bits(16..=31).try_into().unwrap()
    }
}

/// Doorbell Offset
#[repr(transparent)]
pub struct DoorbellOffset(u32);
impl DoorbellOffset {
    /// Returns the offset of the Doorbell Array from the MMIO base.
    #[must_use]
    pub fn get(&self) -> u32 {
        self.0
    }
}

/// Runtime Register Space Offset
#[repr(transparent)]
pub struct RuntimeRegisterSpaceOffset(u32);
impl RuntimeRegisterSpaceOffset {
    /// Returns the offset of the Runtime Registers from the MMIO base.
    #[must_use]
    pub fn get(&self) -> u32 {
        self.0
    }
}
