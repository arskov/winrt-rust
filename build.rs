fn main() {
    println!("cargo:rerun-if-changed=src/win_example.idl");
    let metadata_dir = format!("{}\\System32\\WinMetadata", env!("windir"));
    let mut command = std::process::Command::new("midlrt.exe");

    command.args([
        "/winrt",
        "/nomidl",
        "/h",
        "nul",
        "/metadata_dir",
        &metadata_dir,
        "/reference",
        &format!("{metadata_dir}\\Windows.Foundation.winmd"),
        "/winmd",
        "win_example.winmd",
        "src/win_example.idl",
    ]);

    if !command.status().unwrap().success() {
        panic!("Failed to run midlrt");
    }

    if let Err(error) = windows_bindgen::bindgen([
        "--in",
        "win_example.winmd",
        &metadata_dir,
        "--out",
        "src/bindings.rs",
        "--filter",
        "win_example",
        "--config",
        "implement",
    ]) {
        panic!("{error}");
    }
}
