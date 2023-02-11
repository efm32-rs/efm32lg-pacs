use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());

        let device_file = if env::var_os("CARGO_FEATURE_EFM32LG230").is_some() {
            "src/efm32lg230/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32LG232").is_some() {
            "src/efm32lg232/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32LG280").is_some() {
            "src/efm32lg280/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32LG290").is_some() {
            "src/efm32lg290/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32LG295").is_some() {
            "src/efm32lg295/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32LG330").is_some() {
            "src/efm32lg330/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32LG332").is_some() {
            "src/efm32lg332/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32LG360").is_some() {
            "src/efm32lg360/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32LG380").is_some() {
            "src/efm32lg380/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32LG390").is_some() {
            "src/efm32lg390/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32LG395").is_some() {
            "src/efm32lg395/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32LG840").is_some() {
            "src/efm32lg840/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32LG842").is_some() {
            "src/efm32lg842/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32LG880").is_some() {
            "src/efm32lg880/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32LG890").is_some() {
            "src/efm32lg890/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32LG895").is_some() {
            "src/efm32lg895/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32LG900").is_some() {
            "src/efm32lg900/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32LG940").is_some() {
            "src/efm32lg940/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32LG942").is_some() {
            "src/efm32lg942/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32LG980").is_some() {
            "src/efm32lg980/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32LG990").is_some() {
            "src/efm32lg990/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32LG995").is_some() {
            "src/efm32lg995/device.x"
        } else { panic!("No device features selected"); };

        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }

    println!("cargo:rerun-if-changed=build.rs");
}

