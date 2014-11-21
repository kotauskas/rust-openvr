use std::io::Command;
use std::io::fs;

#[cfg(target_os = "linux")]
fn main() {
    Command::new("make")
            .arg("-C").arg("modules/oculus_sdk_linux/")
            .status()
            .ok().expect("Failed to build");
    fs::copy(&Path::new("modules/oculus_sdk_linux/LibOVR/Lib/Linux/Release/x86_64/libovr.a"),
             &Path::new(env!("OUT_DIR")).join(Path::new("libovr.a")))
            .ok().expect("Failed to move file");

    println!("cargo:rustc-flags=-L {} -l ovr:static", env!("OUT_DIR"));
}


#[cfg(target_os = "macos")]
fn main() {
    Command::new("xcodebuild")
            .arg("-project")
            .arg("modules/oculus_sdk_mac/LibOVR/Projects/Mac/Xcode/LibOVR.xcodeproj")
            .arg("build")
            .status()
            .ok().expect("Failed to build");
    fs::copy(&Path::new("modules/oculus_sdk_mac/LibOVR/Lib/MacOS/Release/libovr.a"),
             &Path::new(env!("OUT_DIR")).join(Path::new("libovr.a")))
            .ok().expect("Failed to move file");
    println!("cargo:rustc-flags=-L {} -l ovr:static", env!("OUT_DIR"));

}