# Android build (Stage 1b)

Gota runs on Android as a native `NativeActivity` application written in Rust, using the
official Slint Android backend (`backend-android-activity-06`). The UI is the same
`gota-ui` crate used on desktop. There is no WebView.

- Application ID: `io.github.shntsk.gota`
- ABI: `arm64-v8a` (`aarch64-linux-android`)
- Minimum SDK 26, target SDK 34
- Build tool: [`cargo-apk`](https://github.com/rust-mobile/cargo-apk), the tool recommended by
  the Slint documentation. `xbuild` was not chosen: it is installed from git and its CLI is
  less stable for CI.

## Layout

- `apps/android`: `cdylib` crate with the `android_main` entry point and the APK metadata
  (`[package.metadata.android]`). On non-Android hosts it compiles to an empty library, so
  `cargo clippy/test --workspace` keeps working on Linux and Windows.

## Build locally

Requirements: Rust target, JDK 17, Android SDK (platform 34) and NDK.

```sh
rustup target add aarch64-linux-android
cargo install cargo-apk --locked
export ANDROID_HOME=/path/to/Android/Sdk
export ANDROID_NDK_ROOT=$ANDROID_HOME/ndk/<version>

cd apps/android
cargo apk build --lib --target aarch64-linux-android
# APK: target/debug/apk/gota.apk
```

Install and inspect on a device with USB debugging enabled:

```sh
adb install -r target/debug/apk/gota.apk
adb logcat | grep -i -E "gota|rust|slint|AndroidRuntime"
```

CI builds the same APK in the `android` job and publishes the `gota-android-arm64-v8a`
artifact (14 days).

## Device test checklist

1. The app installs and the window opens, filling the screen.
2. Rotating the device resizes the content.
3. Back button closes the app; opening it again works (the process exits on close by design).
4. Sending the app to the background and returning keeps the window working.

## Notes

- This is a debug-signed development APK, not a release build.
- Lifecycle events (surface created/destroyed, resize, pause/resume) are handled by the
  Slint Android backend.
