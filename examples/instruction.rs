use qbdi_sys2::{
    qbdi_addCodeCB, qbdi_addInstrumentedModuleFromAddr, qbdi_alignedFree,
    qbdi_allocateVirtualStack, qbdi_call, qbdi_getGPRState, qbdi_getInstAnalysis, qbdi_initVM,
    qbdi_terminateVM, rword, AnalysisType_QBDI_ANALYSIS_DISASSEMBLY,
    AnalysisType_QBDI_ANALYSIS_INSTRUCTION, FPRState, GPRState, InstPosition_QBDI_PREINST,
    VMAction, VMAction_QBDI_CONTINUE, VMInstanceRef,
};
use std::ptr;

extern "C" fn add(a: rword, b: rword) -> rword {
    a + b
}

unsafe extern "C" fn qbdi_instruction_callback(
    vm: VMInstanceRef,
    _gpr_state: *mut GPRState,
    _fpr_state: *mut FPRState,
    _data: *mut std::os::raw::c_void,
) -> VMAction {
    let instruction = qbdi_getInstAnalysis(
        vm,
        AnalysisType_QBDI_ANALYSIS_INSTRUCTION | AnalysisType_QBDI_ANALYSIS_DISASSEMBLY,
    );
    let address = (*instruction).address;
    let dissassembly = std::ffi::CStr::from_ptr((*instruction).disassembly);
    println!(
        "[{address:#018x}] Executing {}",
        dissassembly.to_string_lossy()
    );
    VMAction_QBDI_CONTINUE
}

#[allow(clippy::fn_to_numeric_cast)]
fn main() {
    // Create VM
    let mut vm: VMInstanceRef = ptr::null_mut();
    unsafe { qbdi_initVM(&mut vm, ptr::null(), ptr::null_mut(), 0) };

    // Allocate some stack space
    let mut stack = ptr::null_mut();
    unsafe { qbdi_allocateVirtualStack(qbdi_getGPRState(vm), 4096, &mut stack) };

    // Add a hook to call before executing each instruction
    unsafe {
        qbdi_addCodeCB(
            vm,
            InstPosition_QBDI_PREINST,
            Some(qbdi_instruction_callback),
            std::ptr::null_mut(),
            0,
        )
    };

    // Add the module containing "add" to the tracing list
    assert!(
        unsafe { qbdi_addInstrumentedModuleFromAddr(vm, add as rword) },
        "Failed to add instrumentation module"
    );

    // Call our function using the VM
    let a = 2;
    let b = 4;
    let mut c = 0;
    println!("Calling add({a}, {b})");
    unsafe { qbdi_call(vm, &mut c, add as rword, 2, a, b) };
    println!("add({a}, {b}) = {c}");
    assert_eq!(c, add(a, b));

    // Deallocate the stack space
    unsafe { qbdi_alignedFree(stack as *mut std::os::raw::c_void) };

    // Shut down and free the VM
    unsafe { qbdi_terminateVM(vm) };
}
