//! Example demonstrating how to host the CLR and execute managed code.
//!
//! This example shows how to:
//! 1. Get the ICLRMetaHost interface
//! 2. Get runtime info for a specific CLR version
//! 3. Get the ICLRRuntimeHost interface
//! 4. Start the CLR
//! 5. Execute a method in the default AppDomain
//!
//! # Prerequisites
//! - .NET Framework 4.x must be installed
//! - A managed assembly with a static method matching the expected signature

use mscoree::{
    CLRCreateInstance, CLSID_CLRMetaHost, CLSID_CLRRuntimeHost, ICLRMetaHost, ICLRRuntimeHost,
    ICLRRuntimeInfo, IID_ICLRRuntimeHost, IID_ICLRRuntimeInfo,
};
use windows::core::{Interface, w};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        // Step 1: Get the meta host
        println!("Getting ICLRMetaHost...");
        let meta_host: ICLRMetaHost = CLRCreateInstance(&CLSID_CLRMetaHost)?;
        println!("  Got ICLRMetaHost");

        // Step 2: Get runtime info for .NET 4.0
        println!("Getting ICLRRuntimeInfo for v4.0.30319...");
        let version = w!("v4.0.30319");
        let mut runtime_info_ptr: *mut core::ffi::c_void = core::ptr::null_mut();
        meta_host
            .GetRuntime(version, &IID_ICLRRuntimeInfo, &mut runtime_info_ptr)
            .ok()?;
        let runtime_info: ICLRRuntimeInfo = ICLRRuntimeInfo::from_raw(runtime_info_ptr);
        println!("  Got ICLRRuntimeInfo");

        // Step 3: Get version string to verify
        let mut version_buffer = [0u16; 256];
        let mut version_len = version_buffer.len() as u32;
        runtime_info
            .GetVersionString(version_buffer.as_mut_ptr(), &mut version_len)
            .ok()?;
        let version_str = String::from_utf16_lossy(&version_buffer[..version_len as usize - 1]);
        println!("  Runtime version: {}", version_str);

        // Step 4: Get ICLRRuntimeHost
        println!("Getting ICLRRuntimeHost...");
        let mut runtime_host_ptr: *mut core::ffi::c_void = core::ptr::null_mut();
        runtime_info
            .GetInterface(
                &CLSID_CLRRuntimeHost,
                &IID_ICLRRuntimeHost,
                &mut runtime_host_ptr,
            )
            .ok()?;
        let runtime_host: ICLRRuntimeHost = ICLRRuntimeHost::from_raw(runtime_host_ptr);
        println!("  Got ICLRRuntimeHost");

        // Step 5: Start the CLR
        println!("Starting CLR...");
        runtime_host.Start().ok()?;
        println!("  CLR started successfully!");

        // Step 6: Execute managed code (requires an actual assembly)
        // Uncomment and modify the following to execute a method:
        //
        // let assembly_path = w!("C:\\path\\to\\your\\assembly.dll");
        // let type_name = w!("YourNamespace.YourClass");
        // let method_name = w!("YourMethod");
        // let argument = w!("Hello from Rust!");
        // let mut return_value: u32 = 0;
        //
        // runtime_host.ExecuteInDefaultAppDomain(
        //     assembly_path,
        //     type_name,
        //     method_name,
        //     argument,
        //     &mut return_value,
        // ).ok()?;
        //
        // println!("Method returned: {}", return_value);

        // Step 7: Stop the CLR
        println!("Stopping CLR...");
        runtime_host.Stop().ok()?;
        println!("  CLR stopped.");

        println!("\nCLR hosting example completed successfully!");
    }

    Ok(())
}
