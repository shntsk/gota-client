# Arquitetura do Gota

Gota é um cliente nativo, independente e não oficial para o Fluxer, em Rust com Slint.
Alvos: Windows x86_64, Linux x86_64 e Android arm64-v8a. Não há suporte a macOS.
Sem Electron, Tauri, WebView ou interface em HTML/CSS.

## Princípios

- Uma única interface Slint compartilhada, com layouts adaptáveis por largura.
- Lógica em Rust, sem dependência de UI no núcleo.
- Código específico de plataforma isolado em `apps/` e `platform/`, nunca em componentes visuais.
- Nenhuma rede nem acesso a disco na thread de renderização.
- Cada crate é criada somente quando o estágio correspondente precisa dela.

## Estrutura de destino

```
gota-client/
├── apps/{desktop,android}
├── crates/{gota-ui,gota-core,gota-fluxer,gota-models,gota-store,gota-media,gota-platform}
├── platform/{windows,linux,android}
├── assets/
├── docs/{policy.md,architecture.md}
├── tools/xtask
└── .github/workflows/ci.yml
```

## Estado atual (Estágio 1a)

| Item | Situação |
|---|---|
| `apps/desktop` (binário `gota`) | existe |
| `crates/gota-ui` (janela Slint mínima) | existe |
| `tools/xtask` (`cargo xtask check`) | existe |
| CI Windows e Linux | existe |
| `apps/android`, `gota-core`, `gota-fluxer`, `gota-models`, `gota-store`, `gota-media`, `gota-platform`, `platform/*`, `assets/` | planejados, ainda não criados |

## Fluxo de dados previsto

```
apps/* ──► gota-ui (Slint) ◄── Command ↓ / ↑ CoreEvent ──► gota-core
                                                      │
                          gota-fluxer · gota-store · gota-media · gota-models
                                          gota-platform (traits) ◄── platform/*
```

A UI roda na thread principal; o runtime assíncrono (Tokio) roda em threads próprias e
atualiza a UI via `upgrade_in_event_loop`.

## Estágios

0 auditoria (feito) · 1a fundação desktop (este) · 1b Android · 2 design system ·
3 UI desktop · 4 UI Android · 5 API e autenticação · 6 gateway · 7 persistência ·
8 mídia · 9 notificações · 10 testes e distribuição.
Materiais de vidro nativos (Mica/Acrylic, `ext-background-effect-v1`, blur de janela no
Android) ficam em estágios próprios posteriores, com spike prévio.

## Política e licença

Uso do Fluxer: `docs/policy.md`. Licença do projeto: GPL-3.0-only.
