// build.rs
// Windows resource compilation (icon, manifest, version info)

fn main() {
    // On Windows, embed a manifest for DPI awareness and require admin rights
    #[cfg(target_os = "windows")]
    {
        // If winres is available, embed version info and icon
        // This is optional; the binary will still build without it
        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rerun-if-changed=assets/icon.ico");

        // Try to use winres for resource embedding (optional dep)
        // If you have an icon at assets/icon.ico, uncomment the following:
        // let mut res = winres::WindowsResource::new();
        // res.set_icon("assets/icon.ico");
        // res.set("FileDescription", "Traffic Monitor - Network Speed Monitor");
        // res.set("ProductName", "Traffic Monitor");
        // res.set("LegalCopyright", "GPL-3.0");
        // res.compile().expect("Failed to compile Windows resource");
    }
}
