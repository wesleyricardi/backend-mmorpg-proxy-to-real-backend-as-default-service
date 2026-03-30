# MMORPG Proxy (source-backend)

Proxy em Rust para interceptar e encaminhar tráfego entre cliente e servidor de jogo, com suporte a:

- encaminhamento de pacotes cliente -> servidor e servidor -> cliente
- pipeline de handlers para mensagens específicas
- sincronização com servidor legado (warmup + runtime)
- runtime interno de mundo/skills
- painel opcional de debug em UI nativa

## Visão Geral

O projeto sobe um servidor local que escuta conexões do cliente de jogo, encaminha pacotes para o servidor real e aplica lógica de domínio em pontos específicos (por exemplo, skills, estado de player, sincronização de mundo).

Fluxo alto nível:

1. Inicializa logger e UI de debug (opcional).
2. Carrega estado de skills a partir de arquivos JSON.
3. Sobe runtime interno do mundo em thread dedicada.
4. Sobe supervisor de sincronização com servidor legado.
5. Inicia servidor socket escutando em `0.0.0.0:LISTEN_PORT`.

## Stack

- Rust 2021
- Tokio
- eframe/egui (debug UI)
- Fern + log + chrono (logging)
- serde/serde_json (serialização)

## Estrutura Principal

Resumo das pastas mais relevantes dentro de `src/`:

- `adapters/`: camada de integração com I/O, handlers de protocolo e sincronização legado.
- `application/`: casos de uso e runtime de mundo.
- `config/`: portas, logger, middleware e roteamento de handlers.
- `debug_ui/`: interface de debug opcional para inspeção de estado.
- `domain/`: regras de negócio (skills, player, itens, validações).
- `protocol/`: estruturas, códigos e transcodificação dos pacotes.
- `state/`: estado compartilhado da aplicação.
- `world_transport/`: mensagens/comandos de transporte entre componentes.

## Requisitos

- Rust toolchain (recomendado via `rustup`)
- Cargo
- Windows (ambiente atual), Linux/macOS também podem funcionar com ajustes

## Dependências Locais Importantes

O `Cargo.toml` referencia uma dependência por caminho local:

- `socket = { path = "socket" }`

Isso significa que a pasta `socket/` (com seu próprio `Cargo.toml`) precisa existir na raiz do projeto.

Sem isso, comandos como `cargo check` e `cargo run` falham com erro de dependência não encontrada.

## Configuração

### 1. Endereços e portas

Arquivo: `src/config/global.rs`

- `LISTEN_PORT`: porta que o proxy abre para o cliente (padrão: `10009`)
- `SERVER_ADDR`: endereço do servidor de jogo de destino (padrão: `127.0.0.1`)
- `SERVER_PORT`: porta do servidor de destino (padrão: `10008`)

### 2. Arquivos de skill

Na inicialização, o sistema tenta carregar JSONs de skill em:

- `config/Skill/{className}.json`

Garanta que esses arquivos existam antes de rodar o projeto.

### 3. Debug UI (opcional)

Variável de ambiente:

- `SOURCE_BACKEND_DEBUG_UI=1` (ou `true`, `yes`, `on`) para habilitar

Comportamento padrão:

- em build de debug: habilitado por padrão
- em build release: desabilitado por padrão

## Como Rodar

### Desenvolvimento

```powershell
cargo run
```

### Release

```powershell
cargo run --release
```

## Logs

- Saída colorida no terminal
- Arquivo de log em `logs/<data>.log`

Exemplo de arquivo:

- `logs/30-03-2026.log`

## Comandos Úteis

```powershell
cargo fmt
cargo clippy --all-targets --all-features
cargo test
```

## Troubleshooting

### Erro: dependência `socket` não encontrada

Confirme se existe a pasta `socket/` na raiz com `socket/Cargo.toml`.

### Erro ao carregar skills na inicialização

Confirme a existência dos arquivos JSON em `config/Skill/` e os nomes esperados.

### Cliente não conecta no proxy

Verifique:

1. porta do proxy (`LISTEN_PORT`)
2. destino legado (`SERVER_ADDR` e `SERVER_PORT`)
3. regras de firewall local

## Estado Atual do Repositório

No estado atual deste workspace, faltam pelo menos:

- pasta local `socket/`
- arquivos de configuração `config/Skill/*.json`

Sem esses itens, a aplicação não inicializa completamente.