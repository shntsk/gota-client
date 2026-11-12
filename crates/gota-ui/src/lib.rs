// SPDX-License-Identifier: GPL-3.0-only

//! Interface Slint compartilhada do Gota.

pub mod meta;

use slint::ComponentHandle;

slint::include_modules!();

/// Cria a janela principal e executa o laço de eventos até que ela seja fechada.
pub fn run() -> Result<(), slint::PlatformError> {
    let window = AppWindow::new()?;
    window.set_app_name(meta::APP_NAME.into());
    window.set_tagline(meta::TAGLINE.into());
    window.set_notice(meta::NOTICE.into());
    window.set_version(meta::VERSION.into());
    window.run()
}
