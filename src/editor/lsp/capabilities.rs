use tower_lsp::lsp_types::{
    CallHierarchyClientCapabilities, ClientCapabilities, CodeLensWorkspaceClientCapabilities,
    CompletionClientCapabilities, CompletionItemCapability, CompletionItemKindCapability,
    CompletionListCapability, DiagnosticClientCapabilities, DiagnosticWorkspaceClientCapabilities,
    DidChangeConfigurationClientCapabilities, DidChangeWatchedFilesClientCapabilities,
    DocumentColorClientCapabilities, GeneralClientCapabilities, HoverClientCapabilities,
    InlayHintWorkspaceClientCapabilities, InlineValueWorkspaceClientCapabilities, InsertTextMode,
    MarkupKind, ParameterInformationSettings, PositionEncodingKind,
    SemanticTokensWorkspaceClientCapabilities, SignatureHelpClientCapabilities,
    SignatureInformationSettings, TextDocumentClientCapabilities,
    TextDocumentSyncClientCapabilities, TypeHierarchyClientCapabilities, WindowClientCapabilities,
    WorkspaceClientCapabilities, WorkspaceFileOperationsClientCapabilities,
    WorkspaceSymbolClientCapabilities,
};

const TRUE: Option<bool> = Some(true);
const FALSE: Option<bool> = Some(false);

// I do not see a need to register capabilities dynamically.
const DYNAMIC_REGISTRATION: Option<bool> = FALSE;

// The markup kinds that we can display.
fn markup_kinds() -> Vec<MarkupKind> {
    // TODO: add markdown?
    vec![MarkupKind::PlainText]
}

// TODO: remove all `None` fields.
pub(super) fn capabilities() -> ClientCapabilities {
    ClientCapabilities {
        workspace: Some(WorkspaceClientCapabilities {
            // TODO: `workspace/applyEdit`
            apply_edit: None,
            workspace_edit: None,
            did_change_configuration: Some(DidChangeConfigurationClientCapabilities {
                dynamic_registration: DYNAMIC_REGISTRATION,
            }),
            did_change_watched_files: Some(DidChangeWatchedFilesClientCapabilities {
                dynamic_registration: DYNAMIC_REGISTRATION,
                // TODO: `workspace/didChangeWatchedFiles`
                relative_pattern_support: None,
            }),
            // TODO: `workspace/symbol`
            symbol: Some(WorkspaceSymbolClientCapabilities {
                dynamic_registration: DYNAMIC_REGISTRATION,
                symbol_kind: None,
                tag_support: None,
                resolve_support: None,
            }),
            // TODO: `workspace/executeCommand`
            execute_command: None,
            workspace_folders: TRUE,
            // TODO: `workspace/configuration`
            configuration: None,
            semantic_tokens: Some(SemanticTokensWorkspaceClientCapabilities {
                // TODO: `workspace/semanticTokens/refresh`
                refresh_support: None,
            }),
            code_lens: Some(CodeLensWorkspaceClientCapabilities {
                // TODO: `workspace/codeLens/refresh`
                refresh_support: None,
            }),
            file_operations: Some(WorkspaceFileOperationsClientCapabilities {
                dynamic_registration: DYNAMIC_REGISTRATION,
                // TODO: `workspace/didCreateFiles`
                did_create: None,
                // TODO: `workspace/willCreateFiles`
                will_create: None,
                // TODO: `workspace/didRenameFiles`
                did_rename: None,
                // TODO: `workspace/willRenameFiles`
                will_rename: None,
                // TODO: `workspace/didDeleteFiles`
                did_delete: None,
                // TODO: `workspace/willDeleteFiles`
                will_delete: None,
            }),
            inline_value: Some(InlineValueWorkspaceClientCapabilities {
                // TODO: `workspace/inlineValue/refresh`
                refresh_support: None,
            }),
            inlay_hint: Some(InlayHintWorkspaceClientCapabilities {
                // TODO: `workspace/inlayHint/refresh`
                refresh_support: None,
            }),
            diagnostic: Some(DiagnosticWorkspaceClientCapabilities {
                // TODO: `workspace/diagnostic/refresh`
                refresh_support: None,
            }),
        }),
        text_document: Some(TextDocumentClientCapabilities {
            synchronization: Some(TextDocumentSyncClientCapabilities {
                dynamic_registration: DYNAMIC_REGISTRATION,
                // TODO: `textDocument/willSave`
                will_save: None,
                // TODO: `textDocument/willSaveWaitUntil`
                will_save_wait_until: None,
                // TODO: `textDocument/didSave`
                did_save: None,
            }),
            completion: Some(CompletionClientCapabilities {
                dynamic_registration: DYNAMIC_REGISTRATION,
                // TODO: `textDocument/completion`
                completion_item: Some(CompletionItemCapability {
                    snippet_support: None,
                    commit_characters_support: None,
                    documentation_format: None,
                    deprecated_support: None,
                    preselect_support: None,
                    tag_support: None,
                    insert_replace_support: None,
                    resolve_support: None,
                    insert_text_mode_support: None,
                    label_details_support: None,
                }),
                // TODO: `textDocument/completion`
                completion_item_kind: Some(CompletionItemKindCapability { value_set: None }),
                // TODO: `textDocument/completion`
                context_support: None,
                insert_text_mode: Some(InsertTextMode::AS_IS),
                completion_list: Some(CompletionListCapability {
                    item_defaults: Some(vec![]),
                }),
            }),
            hover: Some(HoverClientCapabilities {
                dynamic_registration: DYNAMIC_REGISTRATION,
                content_format: Some(markup_kinds()),
            }),
            signature_help: Some(SignatureHelpClientCapabilities {
                dynamic_registration: DYNAMIC_REGISTRATION,
                signature_information: Some(SignatureInformationSettings {
                    documentation_format: Some(markup_kinds()),
                    parameter_information: Some(ParameterInformationSettings {
                        label_offset_support: FALSE,
                    }),
                    // TODO: `activeParameter`
                    active_parameter_support: None,
                }),
                // TODO: `textDocument/signatureHelp`
                context_support: None,
            }),
            // TODO: `textDocument/references`
            references: None,
            // TODO: `textDocument/documentHighlight`
            document_highlight: None,
            // TODO: `textDocument/documentSymbol`
            document_symbol: None,
            // TODO: `textDocument/formatting`
            formatting: None,
            // TODO: `textDocument/rangeFormatting`
            range_formatting: None,
            // TODO: `textDocument/onTypeFormatting`
            on_type_formatting: None,
            // TODO: `textDocument/declaration`
            declaration: None,
            // TODO: `textDocument/definition`
            definition: None,
            // TODO: `textDocument/typeDefinition`
            type_definition: None,
            // TODO: `textDocument/implementation`
            implementation: None,
            // TODO: `textDocument/codeAction`
            code_action: None,
            // TODO: `textDocument/codeLens`
            code_lens: None,
            // TODO: `textDocument/documentLink`
            document_link: None,
            color_provider: Some(DocumentColorClientCapabilities {
                dynamic_registration: DYNAMIC_REGISTRATION,
            }),
            // TODO: `textDocument/rename`
            rename: None,
            // TODO: `textDocument/publishDiagnostics`
            publish_diagnostics: None,
            // TODO: `textDocument/foldingRange`
            folding_range: None,
            // TODO: `textDocument/selectionRange`
            selection_range: None,
            // TODO: `textDocument/linkedEditingRange`
            linked_editing_range: None,
            call_hierarchy: Some(CallHierarchyClientCapabilities {
                dynamic_registration: DYNAMIC_REGISTRATION,
            }),
            // TODO: `textDocument/semanticTokens/*`
            semantic_tokens: None,
            // TODO: `textDocument/moniker`
            moniker: None,
            type_hierarchy: Some(TypeHierarchyClientCapabilities {
                dynamic_registration: DYNAMIC_REGISTRATION,
            }),
            // TODO: `textDocument/inlineValue`
            inline_value: None,
            // TODO: `textDocument/inlayHint`
            inlay_hint: None,
            // TODO: `textDocument/formatting`
            diagnostic: Some(DiagnosticClientCapabilities {
                dynamic_registration: DYNAMIC_REGISTRATION,
                // TODO: related documents
                related_document_support: None,
            }),
        }),
        window: Some(WindowClientCapabilities {
            // TODO: `window/workDoneProgress`
            work_done_progress: None,
            // TODO: `window/showMessage`
            show_message: None,
            // TODO: `window/showDocument`
            show_document: None,
        }),
        general: Some(GeneralClientCapabilities {
            // TODO: regex support
            regular_expressions: None,
            // TODO: markdown support?
            markdown: None,
            // TODO: figure out what we want to do on stale requests
            stale_request_support: None,
            position_encodings: Some(vec![PositionEncodingKind::UTF8]),
        }),
        experimental: None,
    }
}
