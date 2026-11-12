# Avisos de terceiros

O Gota é distribuído sob **GPL-3.0-only** (veja `LICENSE`).

## Estado atual (Estágio 1a)

Dependências diretas de execução:

| Componente | Uso | Licença utilizada |
|---|---|---|
| [Slint](https://slint.dev) | Interface gráfica | GPL-3.0-only (uma das alternativas do licenciamento triplo do Slint) |

As dependências transitivas do Slint e do Rust são verificadas pelo `cargo-deny`
(`deny.toml`), que aceita somente licenças compatíveis com GPL-3.0-only.

## Fluxer

O Gota é um projeto independente e não oficial. Ele **não** inclui código, logo, ícones
nem outros assets do Fluxer. O código do Fluxer é AGPL-3.0-or-later e seus assets
visuais são CC BY-SA 4.0; nada disso é redistribuído aqui.

## Lista completa

A lista completa e gerada automaticamente de dependências e licenças será produzida no
Estágio 10 (distribuição), antes de qualquer release público.
