<p align="center">
  <strong><a href="https://github.com/shntsk/gota-client">Gota</a></strong>
  <br>
  A native desktop client for Fluxer.
</p>

<p align="center">
  <a href="https://github.com/shntsk/gota-client/actions/workflows/ci.yml">
    <img src="https://github.com/shntsk/gota-client/actions/workflows/ci.yml/badge.svg" alt="CI status">
  </a>
  <a href="./LICENSE">
    <img src="https://img.shields.io/badge/license-GPL--3.0--only-blue.svg" alt="GPL-3.0-only license">
  </a>
  <img src="https://img.shields.io/badge/language-Rust-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/UI-Slint-7B68EE.svg" alt="Slint">
</p>

Gota is an independent, open-source client for [Fluxer](https://fluxer.com), built with **Rust** and **Slint**. The project targets Linux and Windows desktop, with Android support planned.

> Gota is an unofficial third-party project. It is not affiliated with, endorsed by, or sponsored by Fluxer.

## Status

Gota is in early development. The current codebase contains the initial Rust workspace and a minimal Slint desktop application. Fluxer integration and the Android application are not implemented yet.

| Platform | Status | Architecture |
| --- | --- | --- |
| Linux | Initial desktop application | x86_64 |
| Windows | Initial desktop application | x86_64 |
| Android | Planned | arm64-v8a |

## Build

Install the Rust toolchain specified in [`rust-toolchain.toml`](./rust-toolchain.toml), then run the desktop application:

```sh
cargo run -p gota-desktop
```

Run the project checks:

```sh
cargo xtask check
```

## Continuous integration

GitHub Actions is configured to check and build the Linux and Windows desktop targets on pushes, pull requests, and manual runs.

Successful runs publish development artifacts:

- `gota-linux-x86_64` — `.tar.gz`
- `gota-windows-x86_64` — `.zip`

Download them from [GitHub Actions](https://github.com/shntsk/gota-client/actions). These packages are development builds, not release installers.

## License

Gota is licensed under the [GNU General Public License v3.0 only](./LICENSE). Third-party components and their licenses are listed in [`THIRD_PARTY_NOTICES.md`](./THIRD_PARTY_NOTICES.md).
