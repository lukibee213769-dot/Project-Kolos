use std::path::PathBuf;

fn main() {
    let output_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR is set by Cargo"));
    let kernel = PathBuf::from(
        std::env::var_os("CARGO_BIN_FILE_KERNEL_kernel")
            .expect("kernel artifact dependency provides the kernel binary"),
    );
    let image = output_dir.join("kolos-uefi.img");

    bootloader::UefiBoot::new(&kernel)
        .create_disk_image(&image)
        .expect("create bootable UEFI disk image");

    println!("cargo:rustc-env=KOLOS_UEFI_IMAGE={}", image.display());
}
