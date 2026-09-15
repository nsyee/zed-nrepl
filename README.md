# zed-nrepl

A [Zed](https://zed.dev) extension that launches the
[ts-nrepl](https://github.com/nsyee/ts-nrepl) Language Server
(`src/lsp-server.ts`, JSON-RPC over stdio) so you can evaluate TypeScript /
JavaScript in a running nREPL server directly from the editor.

The extension is a thin launcher written in Rust and compiled to WebAssembly.
It does not bundle ts-nrepl or Node.js; both come from your machine.

> **Important:** the language server only *connects* to an nREPL server that
> is already running. It never starts one. Start the nREPL server yourself
> (see below) before evaluating anything.

## Prerequisites

1. **Node.js >= 22.18** on your `PATH` (or an explicit path in settings, see below).
2. **A local clone of ts-nrepl**:

   ```sh
   git clone https://github.com/nsyee/ts-nrepl.git
   ```

   No build step is required; the LSP is started straight from
   `src/lsp-server.ts` (Node strips the types at runtime).

3. **A running nREPL server** hosting the evaluation target. From the ts-nrepl
   checkout:

   ```sh
   npm start                          # VM target (Node sandbox), TCP 7888
   NREPL_TARGET=browser npm start     # Browser target, TCP 7888 + WebSocket 7889
   ```

## Installation (dev extension)

1. Install Rust and the WebAssembly target:

   ```sh
   rustup target add wasm32-wasip2
   ```

2. Clone this repository:

   ```sh
   git clone https://github.com/nsyee/zed-nrepl.git
   ```

3. In Zed, open the command palette and run `zed: install dev extension`,
   then choose the `zed-nrepl` directory. Zed compiles the extension for you.

Verify the build manually with:

```sh
cargo build --release --target wasm32-wasip2
```

## Configuration

Add an `lsp.nREPL` block to your Zed `settings.json`
(`zed: open settings`). The language server name is `nREPL`.

```jsonc
{
  "lsp": {
    "nREPL": {
      // Optional: explicit Node.js binary. If omitted, `node` is looked up
      // in your shell PATH. Set this when Zed cannot find node (nvm, mise,
      // volta, ... users often need it).
      "binary": {
        "path": "/usr/local/bin/node"
      },
      // Required: absolute path to src/lsp-server.ts in your ts-nrepl clone.
      "settings": {
        "serverPath": "/absolute/path/to/ts-nrepl/src/lsp-server.ts"
      },
      // Optional: passed verbatim to the LSP as initializationOptions.
      "initialization_options": {
        "host": "127.0.0.1",
        "port": 7888,
        "autoConnect": true
      }
    }
  }
}
```

| Key | Required | Meaning |
| --- | --- | --- |
| `lsp.nREPL.settings.serverPath` | yes | Absolute path to `<ts-nrepl>/src/lsp-server.ts`. |
| `lsp.nREPL.binary.path` | no | Node.js executable. Defaults to `node` found on `PATH`. |
| `lsp.nREPL.binary.arguments` | no | Overrides the full argument list (advanced; normally leave unset). Default is `[<serverPath>, "--stdio"]`. |
| `lsp.nREPL.binary.env` | no | Extra environment variables for the server process. |
| `lsp.nREPL.initialization_options.host` | no | nREPL host. Defaults to `127.0.0.1` on the LSP side. |
| `lsp.nREPL.initialization_options.port` | no | nREPL port. Defaults to `7888` on the LSP side. |
| `lsp.nREPL.initialization_options.autoConnect` | no | When `true` and both `host` and `port` are set, connect during `initialize`. |

The resulting launch command is:

```
<node> <serverPath> --stdio
```

The extension enables the server for the `TypeScript`, `TSX` and `JavaScript`
languages.

## Usage

1. Start the nREPL server (`npm start` in ts-nrepl).
2. Open a TypeScript or JavaScript file in Zed.
3. Select the expression you want to evaluate.
4. Open code actions (`cmd+.` on macOS, `ctrl+.` on Linux) and pick:
   - **Connect to nREPL** if not connected yet (runs `nrepl/connect`).
   - **Evaluate form (nREPL)** to evaluate the selection (runs `nrepl/eval`).
5. Results are shown as a notification (`window/showMessage`), e.g. `=> 3`.
   Errors are published as diagnostics on the evaluated range and also shown
   as an error notification.

Code actions are only offered when the selection is non-empty. If the
connection drops, the next code action offers **Connect to nREPL** again.

## UI limitations

Zed has no equivalent of VS Code's Webview panels, so this extension cannot
provide a Calva-style persistent output/REPL view or an interactive prompt.
All feedback goes through Zed's notifications and diagnostics.

## Troubleshooting

- **"`node` was not found in PATH"** – set `lsp.nREPL.binary.path` to your
  Node.js binary (`which node` in a terminal).
- **"`lsp.nREPL.settings.serverPath` is not set"** – add the absolute path to
  `src/lsp-server.ts`.
- **"nREPL: connection failed"** – the nREPL server is not running or is
  listening on a different host/port than configured.
- Open `zed: open log` to see the launch command and any startup errors.

## License

MIT
