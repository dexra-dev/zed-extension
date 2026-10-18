use std::env;

use schemars::JsonSchema;
use serde::Deserialize;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

/// The Dexra stdio bridge on npm; it forwards every MCP message to https://api.dexra.dev/mcp.
const PACKAGE_NAME: &str = "@dexra-dev/mcp";
const SERVER_PATH: &str = "node_modules/@dexra-dev/mcp/bin/dexra-mcp.js";

struct DexraExtension;

#[derive(Debug, Default, Deserialize, JsonSchema)]
struct DexraSettings {
    /// Dexra API key (dxr_...) from https://dexra.dev/app/keys. Optional: without it the free
    /// tools work and paid tools are paid per call in USDC over x402.
    #[serde(default)]
    api_key: Option<String>,
}

impl zed::Extension for DexraExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let latest_version = zed::npm_package_latest_version(PACKAGE_NAME)?;
        let installed_version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if installed_version.as_deref() != Some(latest_version.as_ref()) {
            zed::npm_install_package(PACKAGE_NAME, &latest_version)?;
        }

        let settings = ContextServerSettings::for_project("dexra-mcp-server", project)?;
        let settings: DexraSettings = match settings.settings {
            Some(value) => serde_json::from_value(value).map_err(|e| e.to_string())?,
            None => DexraSettings::default(),
        };
        let env = settings
            .api_key
            .map(|key| key.trim().to_owned())
            .filter(|key| !key.is_empty())
            .map(|key| vec![("DEXRA_API_KEY".to_owned(), key)])
            .unwrap_or_default();

        let server_path = env::current_dir()
            .map_err(|e| e.to_string())?
            .join(SERVER_PATH)
            .to_string_lossy()
            .to_string();

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![server_path],
            env,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema = serde_json::to_string(&schemars::schema_for!(DexraSettings))
            .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(DexraExtension);
