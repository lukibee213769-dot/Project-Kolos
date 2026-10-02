use std::path::{Path, PathBuf};
use std::process::Command;

const KERNEL_TARGET: &str = "x86_64-unknown-none";

fn main() {
    let output_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR is set by Cargo"));
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Cargo only sets CARGO_BIN_FILE_* for artifact dependencies built for the
    // *same* target as the depending package. The runner is a host binary while
    // the kernel is a bare-metal x86_64-unknown-none image, so we invoke Cargo
    // directly and read the artifact out of the target directory.
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let release = std::env::var_os("PROFILE").as_deref() == Some("release".as_ref());

    // The nested build must not share the target directory: the outer Cargo
    // holds an exclusive lock on it for the whole build, so reusing it would
    // deadlock. The kernel therefore lands in `target/kernel-build/`.
    let kernel_target_dir = workspace_root.join("target").join("kernel-build");

    let mut command = Command::new(&cargo);
    command
        .current_dir(&workspace_root)
        .env("CARGO_TARGET_DIR", &kernel_target_dir)
        .args(["build", "-p", "kernel", "--target", KERNEL_TARGET]);
    if release {
        command.arg("--release");
    }
    let status = command
        .status()
        .unwrap_or_else(|e| panic!("failed to run `{cargo} build -p kernel`: {e}"));
    assert!(
        status.success(),
        "`{cargo} build -p kernel` failed with {status}"
    );

    let kernel = find_kernel_artifact(&kernel_target_dir, !release);
    println!("cargo:rerun-if-changed=kernel/src/main.rs");
    println!("cargo:rerun-if-changed=kernel/Cargo.toml");

    let image = output_dir.join("kolos-uefi.img");
    bootloader::UefiBoot::new(&kernel)
        .create_disk_image(&image)
        .expect("create bootable UEFI disk image");

    println!("cargo:rustc-env=KOLOS_UEFI_IMAGE={}", image.display());
}

fn find_kernel_artifact(target_dir: &Path, debug: bool) -> PathBuf {
    let kernel = target_dir
        .join(KERNEL_TARGET)
        .join(if debug { "debug" } else { "release" })
        .join("kernel");
    assert!(
        kernel.exists(),
        "kernel artifact not found at {} - was the kernel built for {KERNEL_TARGET}?",
        kernel.display()
    );
    kernel
}
