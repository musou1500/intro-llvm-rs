extern crate llvm_sys;

use llvm_sys::core::*;
use llvm_sys::execution_engine::*;
use llvm_sys::target::*;
use std::ffi::CString;
use std::mem;

fn main() {
    unsafe {
        let context = LLVMContextCreate();
        let builder = LLVMCreateBuilderInContext(context);
        let module = LLVMModuleCreateWithName(CString::new("main").unwrap().as_ptr());
        let i64t = LLVMInt64TypeInContext(context);

        let mut argts = [];
        let function_type = LLVMFunctionType(i64t, argts.as_mut_ptr(), argts.len() as u32, 0);

        let function = LLVMAddFunction(
            module,
            CString::new("main").unwrap().as_ptr(),
            function_type,
        );
        let bb = LLVMAppendBasicBlockInContext(
            context,
            function,
            CString::new("entry").unwrap().as_ptr(),
        );
        LLVMPositionBuilderAtEnd(builder, bb);

        let a = LLVMBuildAlloca(
            builder,
            LLVMInt64TypeInContext(context),
            CString::new("a").unwrap().as_ptr(),
        );
        LLVMBuildStore(builder, LLVMConstInt(i64t, 32, 1), a);
        let b = LLVMBuildAlloca(
            builder,
            LLVMInt64TypeInContext(context),
            CString::new("b").unwrap().as_ptr(),
        );
        LLVMBuildStore(builder, LLVMConstInt(i64t, 23, 1), b);

        let a_val = LLVMBuildLoad(builder, a, CString::new("a_val").unwrap().as_ptr());
        let b_val = LLVMBuildLoad(builder, b, CString::new("b_val").unwrap().as_ptr());

        let sum = LLVMBuildAdd(builder, a_val, b_val, CString::new("sum").unwrap().as_ptr());
        LLVMBuildRet(builder, sum);

        LLVMDisposeBuilder(builder);
        LLVMDumpModule(module);

        let mut engine = mem::uninitialized();
        let mut out = mem::zeroed();

        LLVMLinkInMCJIT();
        LLVM_InitializeNativeTarget();
        LLVM_InitializeNativeAsmPrinter();

        LLVMCreateExecutionEngineForModule(&mut engine, module, &mut out);
        let addr = LLVMGetFunctionAddress(engine, CString::new("main").unwrap().as_ptr());
        let f: extern "C" fn() -> u64 = mem::transmute(addr);
        let res = f();
        println!("result {}", res);
        LLVMDisposeExecutionEngine(engine);
        LLVMContextDispose(context);
    }
}
