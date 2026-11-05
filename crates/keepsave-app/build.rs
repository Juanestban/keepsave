//! Puts the icon and version info into the Windows `.exe`.

fn main() {
  println!("cargo:rerun-if-changed=assets/icon/keepsave.ico");
  #[cfg(windows)]
  if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
    let mut resource = winresource::WindowsResource::new();
    resource.set_icon("assets/icon/keepsave.ico").set("FileDescription", "KeepSave").set("ProductName", "KeepSave");
    resource.compile().expect("compile the Windows resources (needs the Windows SDK's rc.exe)");
  }
}
