mod natives;
mod types;

use std::ffi::CStr;
use std::fs::File;

use ctor::ctor;
use detour::static_detour;
use log::LevelFilter;
use natives::*;
use simplelog::WriteLogger;
use types::*;

#[ctor]
unsafe fn init() {
    std::thread::spawn(|| {
        WriteLogger::init(
            LevelFilter::Info,
            simplelog::Config::default(),
            File::create("ret.log").unwrap(),
        )
        .unwrap();

        CallScriptedHook
            .initialize(CALL_SCRIPTED.clone(), call_hook)
            .unwrap()
            .enable()
            .unwrap();
    });
}

static_detour! {
  static CallScriptedHook: unsafe extern "C" fn(*const Function, *mut StackFrame, *mut u8, *const RTTIType);
}

fn call_hook(this: *const Function, frame: *mut StackFrame, result: *mut u8, ret_type: *const RTTIType) {
    unsafe {
        let name = (*this).full_name;

        log::info!("{}", CStr::from_ptr(GET_POOL_NAME(&name)).to_string_lossy());

        for param in (*this).params.as_slice() {
            let prop = param.as_ref().unwrap();
            let ty_name = (*prop.typ).get_type();
            let prop_name = CStr::from_ptr(GET_POOL_NAME(&prop.name)).to_string_lossy();

            log::info!("{}: {:?}", prop_name, ty_name);
        }

        CallScriptedHook.call(this, frame, result, ret_type);
    }
}
