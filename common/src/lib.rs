#![no_std]

use winapi::um::winioctl::{FILE_DEVICE_UNKNOWN, METHOD_NEITHER, FILE_ANY_ACCESS};

macro_rules! CTL_CODE {
    ($DeviceType:expr, $Function:expr, $Method:expr, $Access:expr) => {
        ($DeviceType << 16) | ($Access << 14) | ($Function << 2) | $Method
    }
}

pub const IOCTL_PROCESS_READ_REQUEST: u32 = CTL_CODE!(FILE_DEVICE_UNKNOWN, 0x800, METHOD_NEITHER, FILE_ANY_ACCESS);
pub const IOCTL_PROCESS_WRITE_REQUEST: u32 = CTL_CODE!(FILE_DEVICE_UNKNOWN, 0x801, METHOD_NEITHER, FILE_ANY_ACCESS);
pub const IOCTL_PROCESS_PROTECT_REQUEST: u32 = CTL_CODE!(FILE_DEVICE_UNKNOWN, 0x802, METHOD_NEITHER, FILE_ANY_ACCESS);
pub const IOCTL_PROCESS_UNPROTECT_REQUEST: u32 = CTL_CODE!(FILE_DEVICE_UNKNOWN, 0x803, METHOD_NEITHER, FILE_ANY_ACCESS);

pub struct TargetProcess {
    pub process_id: u32,
}