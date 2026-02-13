# mscoree-rs

Rust bindings for mscoree COM interfaces for interacting with CLR and COR runtimes.

This crate provides safe Rust wrappers around the COM interfaces defined in `mscoree.h` and `metahost.h` for hosting the .NET Common Language Runtime (CLR).

## Features

- **CLR Hosting** - Host the .NET CLR in your Rust application
- **Runtime Enumeration** - Enumerate installed .NET runtimes
- **Managed Code Execution** - Execute managed code from native Rust
- **Debugging Interfaces** - Access CLR debugging and diagnostics APIs
- **DAC Support** - Data Access Component interfaces for memory inspection
- **Metadata APIs** - Access .NET metadata and assembly information

## Key Interfaces

| Interface | Description |
|-----------|-------------|
| `ICLRMetaHost` | Entry point for CLR hosting (.NET 4.0+) |
| `ICLRRuntimeInfo` | Information about a specific CLR version |
| `ICLRRuntimeHost` | Runtime hosting interface (.NET 2.0+) |
| `ICorRuntimeHost` | Legacy runtime hosting interface (.NET 1.x) |
| `IXCLRDataProcess` | DAC interface for process inspection |
| `ISOSDacInterface` | SOS debugging extension interfaces |

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mscoree = "0.1"
```

## Quick Start

```rust
use mscoree::{CLRCreateInstance, CLSID_CLRMetaHost, ICLRMetaHost};
use windows::core::PCWSTR;

unsafe {
    // Get the meta host
    let meta_host: ICLRMetaHost = CLRCreateInstance(&CLSID_CLRMetaHost)?;
    
    // Get runtime info for .NET 4.0
    let version = windows::core::w!("v4.0.30319");
    let runtime_info = meta_host.GetRuntime(version)?;
}
```

## Examples

### Host the CLR

```bash
cargo run --example host_clr
```

### Enumerate Assemblies in a .NET Process

```bash
cargo run --example enumerate_assemblies -- <PID>
```

## Requirements

- Windows OS
- .NET Framework 4.x or .NET Core/.NET 5+ (depending on target runtime)
- Rust 2024 edition

## License

This project is licensed under the MIT License - see the [LICENSE-MIT](LICENSE-MIT) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

