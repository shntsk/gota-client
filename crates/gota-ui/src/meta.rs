// SPDX-License-Identifier: GPL-3.0-only

//! Textos de identificação do aplicativo exibidos na janela inicial.

pub const APP_NAME: &str = "Gota";
pub const TAGLINE: &str = "Cliente nativo alternativo para o Fluxer";
pub const NOTICE: &str = "Projeto independente e não oficial. Não é afiliado ao Fluxer.";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_is_gota() {
        assert_eq!(APP_NAME, "Gota");
    }

    #[test]
    fn notice_declares_unofficial_client() {
        assert!(NOTICE.contains("não oficial"));
    }

    #[test]
    fn version_looks_like_semver() {
        let parts: Vec<&str> = VERSION.split('.').collect();
        assert_eq!(parts.len(), 3);
        assert!(parts.iter().all(|p| p.parse::<u32>().is_ok()));
    }
}
