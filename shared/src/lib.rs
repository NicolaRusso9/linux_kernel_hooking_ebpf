#![no_std]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HookingFullEvent {
    pub uid: u32,
    pub pid: u32,
    pub _pad: [u8; 3],      // Padding for alignment
    pub comm: [u8; 16],     // Executed command (ex. "bash", "gedit", "passwd")
    pub data: [u8; 64],     // Output of the hooked function (e.g. "ls -l /home/user")
}