use error_set::error_set;
use std::io;
use tower_lsp::jsonrpc;

error_set! {
    Error = {
        #[display("IO Error: {0}")]
        Io(io::Error),
        #[display("Language Server Error: {0}")]
        Rpc(jsonrpc::Error),
    };
}
