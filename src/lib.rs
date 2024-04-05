#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![doc = include_str!("../README.MD")]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ptr;

    use super::*;

    extern "C" fn add(a: rword, b: rword) -> rword {
        a + b
    }

    #[test]
    fn test_add() {
        let mut vm: VMInstanceRef = ptr::null_mut();
        unsafe { qbdi_initVM(&mut vm, ptr::null(), ptr::null_mut(), 0) };
        let mut stack = ptr::null_mut();
        unsafe { qbdi_allocateVirtualStack(qbdi_getGPRState(vm), 4096, &mut stack) };
        #[allow(clippy::fn_to_numeric_cast)]
        let add_addr = add as rword;
        assert!(
            unsafe { qbdi_addInstrumentedModuleFromAddr(vm, add_addr) },
            "Failed to add instrumentation module"
        );
        let a = 2;
        let b = 4;
        let mut c = 0;
        unsafe { qbdi_call(vm, &mut c, add_addr, 2, a, b) };
        assert_eq!(c, add(a, b));
        unsafe { qbdi_alignedFree(stack as *mut std::os::raw::c_void) };
        unsafe { qbdi_terminateVM(vm) };
    }
}
