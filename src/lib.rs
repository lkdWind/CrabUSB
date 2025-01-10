#![no_std]
#![feature(allocator_api, let_chains)]

use abstractions::{PlatformAbstractions, USBSystemConfig};
use alloc::sync::Arc;

extern crate alloc;

pub mod abstractions;
pub mod driver;
pub mod host;
pub mod usb;

pub struct USBSystem<O>
where
    O: PlatformAbstractions,
{
    config: Arc<USBSystemConfig<O>>,
}
