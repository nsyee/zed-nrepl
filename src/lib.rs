//! Zed extension that launches the ts-nrepl Language Server
//! (`<ts-nrepl>/src/lsp-server.ts`, JSON-RPC over stdio).
//!
//! The language server only connects to an already running nREPL server; it
//! does not start one. See README.md for the required `settings.json` keys.

use zed_extension_api::{
    self as zed, serde_json, settings::LspSettings, LanguageServerId, Result, Worktree,
};

/// Name of the language server as declared in `extension.toml`
/// (`[language_servers.nrepl] name = "nREPL"`). This is also the key users
/// write under `lsp` in their `settings.json`.
const LANGUAGE_SERVER_NAME: &str = "nREPL";

/// Key under `lsp.nREPL.settings` holding the absolute path to
/// `<ts-nrepl>/src/lsp-server.ts`.
const SERVER_PATH_KEY: &str = "serverPath";

const STDIO_FLAG: &str = "--stdio";

struct NreplExtension;

fn resolve_node(settings: &LspSettings, worktree: &Worktree) -> Result<String> {
    if let Some(path) = settings
        .binary
        .as_ref()
        .and_then(|binary| binary.path.clone())
        .filter(|path| !path.is_empty())
    {
        return Ok(path);
    }

    worktree.which("node").ok_or_else(|| {
        format!(
            "nREPL: `node` was not found in PATH. Set `lsp.{LANGUAGE_SERVER_NAME}.binary.path` \
             in settings.json to the absolute path of your Node.js (>= 22.18) binary."
        )
    })
}

fn resolve_server_path(settings: &LspSettings) -> Result<String> {
    settings
        .settings
        .as_ref()
        .and_then(|value| value.get(SERVER_PATH_KEY))
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
        .filter(|path| !path.is_empty())
        .ok_or_else(|| {
            format!(
                "nREPL: `lsp.{LANGUAGE_SERVER_NAME}.settings.{SERVER_PATH_KEY}` is not set. \
                 Point it to the absolute path of `src/lsp-server.ts` in your ts-nrepl checkout."
            )
        })
}

impl zed::Extension for NreplExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let settings = LspSettings::for_worktree(LANGUAGE_SERVER_NAME, worktree)?;
        let command = resolve_node(&settings, worktree)?;

        let args = match settings
            .binary
            .as_ref()
            .and_then(|binary| binary.arguments.clone())
        {
            Some(arguments) => arguments,
            None => vec![resolve_server_path(&settings)?, STDIO_FLAG.to_owned()],
        };

        let mut env = worktree.shell_env();
        if let Some(extra) = settings
            .binary
            .as_ref()
            .and_then(|binary| binary.env.clone())
        {
            env.extend(extra);
        }

        Ok(zed::Command { command, args, env })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree(LANGUAGE_SERVER_NAME, worktree)?;
        Ok(settings.initialization_options)
    }
}

zed::register_extension!(NreplExtension);
