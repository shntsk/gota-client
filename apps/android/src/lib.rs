// SPDX-License-Identifier: GPL-3.0-only

//! Android entry point for Gota.

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).expect("failed to initialize the Slint Android backend");
    let result = gota_ui::run();
    // Slint accepts a single platform per process, so a second activity instance in the
    // same process could not start. Exiting here guarantees a clean start on the next launch.
    std::process::exit(if result.is_ok() { 0 } else { 1 });
}
