# Gota

Cliente nativo, **independente e não oficial**, para o [Fluxer](https://fluxer.com).
Escrito em Rust com [Slint](https://slint.dev). Sem Electron, Tauri ou WebView.

> O Gota não é afiliado ao Fluxer e não usa a logo, os ícones nem os assets dele.
> O nome visual "Gota" é provisório.

**Plataformas:** Windows x86_64, Linux x86_64 e (futuramente) Android arm64-v8a.
**Licença:** GPL-3.0-only (`LICENSE`). **Política de uso do Fluxer:** `docs/policy.md`.

## Estado

Estágio 1a: fundação desktop. O app apenas abre uma janela Slint mínima. Ainda não há
login, mensagens, rede nem Android. Veja `docs/architecture.md`.

## Requisitos

- Rust estável (instalado via [rustup](https://rustup.rs); o `rust-toolchain.toml` cuida do resto).
- Linux: pacotes de desenvolvimento de fontconfig, xkbcommon, Wayland/X11 e OpenGL/EGL.
  Em Debian/Ubuntu:
  `sudo apt install pkg-config libfontconfig1-dev libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev libx11-dev libxcb1-dev libgl1-mesa-dev libegl1-mesa-dev`
- Windows: toolchain MSVC (Visual Studio Build Tools).
- Opcional: `cargo install cargo-deny --locked`.

## Comandos

```sh
cargo run -p gota-desktop      # abre a janela (binário "gota")
cargo xtask check              # fmt --check, clippy -D warnings, test e cargo deny check
cargo xtask check --skip-deny  # o mesmo, sem o cargo-deny (usado no CI)
```

Na primeira compilação o Cargo gera o `Cargo.lock`; faça commit dele (é um aplicativo).

## CI

`.github/workflows/ci.yml` roda em push, pull request e manualmente. Gera os artifacts
`gota-linux-x86_64` (tar.gz) e `gota-windows-x86_64` (zip), retidos por 14 dias. São
pacotes simples para teste, não instaladores.
