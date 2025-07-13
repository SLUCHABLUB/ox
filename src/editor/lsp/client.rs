use crate::editor::lsp::capabilities;
use crate::editor::lsp::Result;
use async_lsp_client::{LspServer, ServerMessage};
use std::env::current_dir;
use std::ffi::OsStr;
use std::mem::forget;
use std::process;
use std::sync::LazyLock;
use tokio::runtime::{Builder, Runtime};
use tokio::sync::mpsc::Receiver;
use tower_lsp::lsp_types::{ClientInfo, InitializeParams, Url, WorkspaceFolder};

// TODO: should we just use `#[tokio::main]` instead?
static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    // TODO: should the error be handled? `#[tokio::main]` just unwraps.
    let runtime = Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Unable to start tokio runtime");

    forget(runtime.enter());

    runtime
});

/// An LSP (Language server Protocol) client.
pub struct Client {
    // TODO: Make the language server for the currently open file displayable in the status bar.
    /// The name of the server. It can be displayed in the status bar.
    pub server_name: String,
    /// The language name.
    pub language: Box<str>,
    /// The connected language server.
    pub server: LspServer,
    /// The messages from the server.
    pub messages: Receiver<ServerMessage>,
}

impl Client {
    pub fn new(executable: &OsStr, language: Box<str>) -> Result<Client> {
        // Make sure the runtime is initialised before we start the server.
        LazyLock::force(&RUNTIME);

        // TODO: allow for arguments to the lsp server
        let arguments = [];
        let (server, messages) = LspServer::new(executable, arguments);

        let workspace_folders = current_dir().ok().and_then(|path| {
            let folder = WorkspaceFolder {
                uri: Url::from_file_path(&path).ok()?,
                name: path
                    .file_name()
                    .unwrap_or(path.as_os_str())
                    .to_string_lossy()
                    .into_owned(),
            };

            Some(vec![folder])
        });

        let initialise_params = InitializeParams {
            process_id: Some(process::id()),
            #[expect(deprecated)]
            root_path: None,
            // #[expect(deprecated)]
            root_uri: None,
            initialization_options: None,
            capabilities: capabilities(),
            // TODO: allow tracing?
            trace: None,
            workspace_folders,
            client_info: Some(ClientInfo {
                name: env!("CARGO_PKG_NAME").to_owned(),
                version: Some(env!("CARGO_PKG_VERSION").to_owned()),
            }),
            // TODO: send locale info to the lsp server
            locale: None,
        };

        let result = RUNTIME.block_on(async {
            let result = server.initialize(initialise_params).await;

            server.initialized().await;

            result
        })?;

        let server_name = result.server_info.map_or_else(
            || executable.to_string_lossy().into_owned(),
            |server| server.name,
        );

        Ok(Client {
            server_name,
            language,
            server,
            messages,
        })
    }
}
