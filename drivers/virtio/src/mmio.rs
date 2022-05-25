//! Ref: linux/include/uapi/linux/virtio_mmio.h

use enumflags2::bitflags;
use enumflags2::BitFlags;
use enumflags2::make_bitflags;
use core::sync::atomic::AtomicU32;
use volatile::Volatile;
use volatile_register::RO;
use volatile_register::RW;
use volatile_register::WO;

#[repr(C)]
pub struct VirtioHeader {
    /// Magic value
    pub magic: RO<u32>,
    /// Device version number
    ///
    /// Legacy device returns value 0x1.
    pub version: RO<u32>,
    /// Virtio Subsystem Device ID
    pub device_id: RO<u32>,
    /// Virtio Subsystem Vendor ID
    pub vendor_id: u32,
    /// Flags representing features the device supports
    pub device_features: RO<u32>,
    /// Device (host) features word selection
    device_features_sel: u32,
    /// Reserved
    reserved0: u64,
    /// Flags representing device features understood and activated by the driver
    pub driver_features: u32,
    /// Activated (guest) features word selection
    pub driver_features_sel: u32,
    /// Guest page size (legacy)
    ///
    /// The driver writes the guest page size in bytes to the register during
    /// initialization, before any queues are used. This value should be a
    /// power of 2 and is used by the device to calculate the Guest address
    /// of the first queue page (see QueuePFN).
    pub guest_page_size: WO<u32>,
    /// Reserved
    reserved1: u32,
    /// Virtual queue index
    ///
    /// Writing to this register selects the virtual queue that the following
    /// operations on the QueueNumMax, QueueNum, QueueAlign and QueuePFN
    /// registers apply to. The index number of the first queue is zero (0x0).
    pub queue_sel: u32,
    // Maximum virtual queue size
    ///
    /// Reading from the register returns the maximum size of the queue the
    /// device is ready to process or zero (0x0) if the queue is not available.
    /// This applies to the queue selected by writing to QueueSel and is
    /// allowed only when QueuePFN is set to zero (0x0), so when the queue is
    /// not actively used.
    pub queue_num_max: RO<u32>,
    /// Virtual queue size
    ///
    /// Queue size is the number of elements in the queue. Writing to this
    /// register notifies the device what size of the queue the driver will use.
    /// This applies to the queue selected by writing to QueueSel.
    pub queue_num: u32,
    /// Used Ring alignment in the virtual queue
    ///
    /// Writing to this register notifies the device about alignment boundary
    /// of the Used Ring in bytes. This value should be a power of 2 and
    /// applies to the queue selected by writing to QueueSel.
    pub queue_align: WO<u32>,
    /// Guest physical page number of the virtual queue
    ///
    /// Writing to this register notifies the device about location of the
    /// virtual queue in the Guest’s physical address space. This value is
    /// the index number of a page starting with the queue Descriptor Table.
    /// Value zero (0x0) means physical address zero (0x00000000) and is illegal.
    /// When the driver stops using the queue it writes zero (0x0) to this
    /// register. Reading from this register returns the currently used page
    /// number of the queue, therefore a value other than zero (0x0) means that
    /// the queue is in use. Both read and write accesses apply to the queue
    /// selected by writing to QueueSel.
    pub queue_pfn: RW<u32>,
    /// new interface only
    pub queue_ready: RW<u32>,
    /// Reserved
    reserved2: u64,
    /// Queue notifier
    queue_notify: u32,
    reserved3: [u32; 3],
    /// Interrupt status
    pub interrupt_status: RO<u32>,
    /// Interrupt acknowledge
    pub interrupt_ack: u32,
    /// Reserved
    reserved4: u64,
    pub status: BitFlags<ConfigStatus>,
    reserved5: [u32; 3],
    pub queue_desc_low: u32,
    pub queue_desc_high: u32,
    reserved6: u64,
    pub queue_avail_low: u32,
    pub queue_avail_high: u32,
    reserved7: u64,
    pub queue_used_low: u32,
    pub queue_used_high: u32,

    __r9: [u32; 21],

    pub config_generation: AtomicU32,
}

#[bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConfigStatus {
    Acknowledge = 1,
    FoundDriver = 2,
    DriverOk = 4,
    FeaturesOk = 8,
    NeedsReset = 0x40,
    Failed = 0x80,
}

pub const CONFIG_SPACE_OFFSET: usize = 0x100;

impl VirtioHeader {
    /// Verify a valid header.
    pub fn verify(&self) -> bool {
        self.magic.read() == 0x7472_6976 && self.version.read() == 1 && self.device_id.read() != 0
    }

    /// Get the vendor ID.
    pub fn vendor_id(&self) -> u32 {
        Volatile::new_read_only(&self.vendor_id).read()
    }

    /// Finish initializing the device.
    pub fn finish_init(&mut self) {
        self.status = make_bitflags!(ConfigStatus::{ DriverOk });
    }

    /// Get the max size of queue.
    pub fn max_queue_size(&self) -> u32 {
        self.queue_num_max.read()
    }

    /// Notify device.
    pub fn notify(&mut self, queue: u32) {
        Volatile::new_write_only(&mut self.queue_notify).write(queue);
    }

    /// Acknowledge interrupt and return true if success.
    pub fn ack_interrupt(&mut self) -> bool {
        let interrupt = self.interrupt_status.read();
        if interrupt != 0 {
            Volatile::new_write_only(&mut self.interrupt_ack).write(interrupt);
            true
        } else {
            false
        }
    }

    /// Set queue.
    pub fn queue_set(&mut self, queue: u32, size: u32, align: u32, pfn: u32) {
        Volatile::new_write_only(&mut self.queue_sel).write(queue);
        Volatile::new_write_only(&mut self.queue_num).write(size);
//        self.queue_align.write(align);
//        self.queue_pfn.write(pfn);
    }

    /// Read device features.
    fn read_device_features(&mut self) -> u64 {
        let mut sel = Volatile::new_write_only(&mut self.device_features_sel);
        sel.write(0);
        let mut device_features_bits = self.device_features.read().into();
        let mut sel = Volatile::new_write_only(&mut self.device_features_sel);
        sel.write(1);
        device_features_bits += (self.device_features.read() as u64) << 32;
        device_features_bits
    }

    /// Write driver features.
    fn write_driver_features(&mut self, driver_features_val: u64) {
        let mut sel = Volatile::new_write_only(&mut self.driver_features_sel);
        let mut driver_features = Volatile::new_write_only(&mut self.driver_features);
        sel.write(0);
        driver_features.write(driver_features_val as u32);
        sel.write(1);
        driver_features.write((driver_features_val >> 32) as u32);
    }
}