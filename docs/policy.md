# Política de uso do Fluxer (Gate G0)

**Status:** aprovado
**Data da decisão:** 11 de novembro de 2026
**Escopo:** Gota, cliente alternativo para Windows, Linux e Android

## 1. Decisão

O desenvolvimento do Gota pode prosseguir. O Fluxer permite expressamente clientes de terceiros, e não é necessária autorização individual da equipe para desenvolver dentro das regras abaixo.

Este documento deve ser revisado sempre que as fontes da seção 2 mudarem.

## 2. Fontes oficiais

| Fonte | Última atualização informada |
|---|---|
| https://fluxer.com/help/are-third-party-clients-allowed | 5 de novembro de 2026 |
| https://fluxer.com/help/third-party-clients | 5 de novembro de 2026 |
| https://docs.fluxer.com (referência da API) | consultar a cada estágio de integração |

O que essas páginas afirmam, em resumo:

- Clientes de terceiros e apps modificados são bem-vindos, e é permitido construir o próprio cliente a partir da documentação pública.
- Nada nos termos ou nas diretrizes da comunidade proíbe um cliente não oficial ou modificado.
- Automação não maliciosa da própria conta com o token de usuário é permitida.
- Usar um cliente de terceiros não gera banimento por si só. As diretrizes da comunidade valem igualmente em qualquer cliente.
- Um cliente que cumpra os padrões do Fluxer pode ser divulgado na página de download (contato indicado na própria página de ajuda).

## 3. Limites de uso

É proibido, mesmo que tecnicamente possível:

- Enviar spam ou organizar raids.
- Coletar (scraping) dados de outras pessoas em volume.
- Contornar banimentos, moderação ou rate limits.
- Usar o cliente para prejudicar pessoas ou o serviço.
- Distribuir ou usar o token de outra pessoa.

O Gota é um cliente interativo para a conta do próprio usuário. Não terá recursos de automação em massa, envio programado em larga escala, nem extração de dados de terceiros.

Regras do projeto derivadas disso:

1. Só chamar endpoints presentes na documentação oficial e no OpenAPI. Nenhum endpoint inventado.
2. Acesso a dados apenas na medida necessária à tela que o usuário está usando. Nada de varredura em segundo plano de guilds, canais ou membros.
3. Enviar um `User-Agent` próprio que identifique o app e a versão, conforme a documentação da API.
4. Ignorar campos desconhecidos nas respostas (a API garante compatibilidade para frente).

## 4. Instâncias de terceiros

A política acima vale para o fluxer.com. Em instâncias operadas por outras pessoas, os administradores definem as regras. O cliente deve descobrir os endpoints pelo documento de descoberta da instância (`/.well-known/fluxer`) e informar isso de forma clara ao usuário. Não assumir que a política do fluxer.com se estende a elas.

## 5. Segurança dos tokens

O token de sessão permite agir como o usuário e ler tudo o que ele pode ler. Por isso:

1. **Armazenamento:** exclusivamente no cofre do sistema (Credential Manager no Windows, Secret Service no Linux, Keystore no Android). Proibido fallback em texto puro.
2. **Memória e logs:** o token nunca aparece em logs, mensagens de erro, relatórios de falha ou saída de `Debug`. Os tipos que o guardam implementam `Debug` com valor redigido.
3. **Destino:** o token só é enviado ao endpoint de API e ao gateway da instância à qual a conta pertence, sempre por HTTPS/WSS. Nunca a terceiros, CDNs, serviços de métricas ou URLs de mídia.
4. **Banco local:** o SQLite não guarda o token. O cache local não é criptografado pelo app, portanto deve conter só o necessário e ser limitado em tamanho.
5. **Logout:** apaga o token do cofre, encerra conexões ativas e remove o cache e os rascunhos da conta. A revogação da sessão no servidor será implementada se existir rota documentada para isso (verificar no Estágio 5a).
6. **Sem telemetria:** nenhum envio de analytics, rastreamento ou coletor de falhas de terceiros.
7. **MFA:** o fluxo de login oferece o segundo fator conforme a API documentada. O app não contorna MFA, CAPTCHA nem verificações de segurança.
8. **Transparência:** o app informa ao usuário que é um cliente não oficial e que ele recebe o token de sessão, em linha com o aviso do próprio Fluxer.

## 6. Rate limits

1. Respeitar integralmente os limites e as respostas de limitação documentados pela API. Nenhuma tentativa de burlar, rotacionar identidade ou distribuir requisições para evitá-los.
2. Em respostas de limitação, aguardar o tempo indicado pelo servidor antes de repetir. Backoff exponencial com jitter para erros transitórios e reconexões.
3. Tratar rate limit como estado normal do cliente: a UI mostra o estado e o usuário continua com controle.
4. Reconexão do gateway com backoff e uso de retomada de sessão, sem laços de reconexão agressivos.
5. Os detalhes técnicos (cabeçalhos, buckets, limites do gateway) serão levantados na documentação no Estágio 5a e registrados aqui antes de implementar o cliente HTTP.

## 7. Marca, licenças e assets

- O código do Fluxer é AGPL-3.0-or-later. O Gota não copia código do cliente oficial. Qualquer trecho derivado exigiria reavaliação de licença antes da inclusão.
- Os assets oficiais (logo, ícones, avatares padrão) são CC BY-SA 4.0 e o nome e a logo têm política de marca própria. O Gota usa identidade visual própria e não usa o logo oficial como identidade do app. A forma de se referir ao Fluxer (nome do app, descrição, ícone) será definida na política de marca antes de qualquer distribuição pública.
- O app deixa claro que é independente e não oficial.

## 8. Itens em aberto

- Ler por inteiro os Termos (https://fluxer.com/terms) e as Diretrizes (https://fluxer.com/guidelines), em especial a seção de spam e abuso de serviço, e registrar aqui qualquer restrição adicional.
- Confirmar a política de marca (`GOVERNANCE.md`, seção "Name and marks") antes de nomear e publicar o app.
- Registrar os detalhes de rate limit (seção 6, item 5) e a rota de revogação de sessão (seção 5, item 5).
- Notificações push: verificar se há mecanismo permitido a clientes de terceiros (Estágio 9).
