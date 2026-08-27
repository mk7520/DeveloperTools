// LSP (Language Server Protocol) module

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LsServer};
use std::sync::Arc;

#[derive(Debug)]
pub struct MaestroLanguageServer {
    client: Option<Client>,
}

impl MaestroLanguageServer {
    pub fn new() -> Self {
        Self { client: None }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for MaestroLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(true),
                    trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                }),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverServerCapabilities::Simple(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let items = vec![
            CompletionItem {
                label: "fn".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                insert_text: Some("fn $0() {}".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "struct".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                insert_text: Some("struct $0 {}".to_string()),
                ..Default::default()
            },
        ];

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: "Hover information".to_string(),
            }),
            range: None,
        }))
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

pub async fn start_lsp_server() -> anyhow::Result<(LsServer, tokio::task::JoinHandle<()>)> {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let server = MaestroLanguageServer::new();
    let (service, socket) = LsServer::new(server);
    let handle = tokio::spawn(tower_lsp::Server::new(stdin, stdout, socket).run());

    Ok((service, handle))
}
