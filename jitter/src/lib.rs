//! A small shared library which connects to small stubs patched into QEMU
//! and provides real-time streaming of things like PCs and memory accesses

#![feature(maybe_uninit_slice, naked_functions)]

mod cannoli_memops;
mod cannoli_internals;

// Re-export `HookType`
pub use cannoli_internals::HookType;

