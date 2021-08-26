use std::mem::transmute;

use ctor::ctor;
use winapi::um::libloaderapi::GetModuleHandleA;

use crate::types::*;

pub type CallScripted = unsafe extern "C" fn(*const Function, *mut StackFrame, *mut u8, *const RTTIType);
pub type GetPoolName = unsafe extern "C" fn(*const CName) -> *const i8;

#[ctor]
static MODULE_ADDRESS: usize = GetModuleHandleA(std::ptr::null()) as usize;

#[ctor]
pub static GET_POOL_NAME: GetPoolName = transmute(MODULE_ADDRESS.clone() + 0x1CD9F0);
#[ctor]
pub static CALL_SCRIPTED: CallScripted = transmute(MODULE_ADDRESS.clone() + 0x22EBE0);
