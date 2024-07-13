# Example of Windows Runtime Component implemented with Rust/WinRT

The sample uses windows-rs crate. The component is described in an IDL file. Logic in `build.rs` generates `bindings.rs` with necessary interfaces and structures. You just need to implement login for `implementations` and with help of `windows-rs` crate and Rust you get a working WinRT component, that you can use in UWP app.
