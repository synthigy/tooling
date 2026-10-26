use crate::ipc;
use rmcp::model::{
    CallToolRequestParams, CallToolResponse, CallToolResult, ContentBlock, Implementation, ListToolsResult,
    PaginatedRequestParams, ServerCapabilities, ServerConfig, Tool,
};
use rmcp::service::RequestContext;
use rmcp::{ErrorData, RoleServer, ServerHandler, ServiceExt};
use serde_json::Value;
use std::sync::Arc;

const TOOLS_JSON: &str = include_str!("../tools.json");
const XSQL_REFERENCE: &str = include_str!("../xsql.md");

const INSTRUCTIONS: &str = "Tools for the Synthigy desktop app (Model · Data · Logs), acting as the human's signed-in identity \
on the engine the window shows. get_app_state names the engine and the active tool; switch_tool changes it. \
Model: read with get_model before editing; model_edit applies a whole batch as one undo step. Edits stay local until \
the human saves the version, and deploying is always the human's call. \
Data: write XSQL (xsql_reference has the language), check it with xsql_check, run it with xsql_run and refine it from \
the response. Keep the finished operations as .xsql files in the project's xsql/ folder: SDK code generators turn them \
into typed functions. data_open shows XSQL to the human in Data; data_workspaces reads what the human wrote there. \
Logs: logs_query reads the engine's logs, e.g. to explain a failed request.";

#[derive(Clone)]
pub struct Server {
    tools: Arc<Vec<Tool>>,
}

impl ServerHandler for Server {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("synthigy", env!("CARGO_PKG_VERSION")))
            .with_instructions(INSTRUCTIONS)
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        Ok(ListToolsResult::with_all_items((*self.tools).clone()))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        if !self.tools.iter().any(|t| t.name == request.name) {
            return Err(ErrorData::invalid_params(format!("unknown tool: {}", request.name), None));
        }
        if request.name == "xsql_reference" {
            return Ok(CallToolResult::success(vec![ContentBlock::text(XSQL_REFERENCE)]).into());
        }
        let args = Value::Object(request.arguments.unwrap_or_default());
        let result = match ipc::request(&request.name, args).await {
            Ok(value) => CallToolResult::success(vec![ContentBlock::text(
                serde_json::to_string_pretty(&value).unwrap_or_default(),
            )]),
            Err(message) => CallToolResult::error(vec![ContentBlock::text(message)]),
        };
        Ok(result.into())
    }
}

/// `synthigy-desktop --mcp`: MCP over stdio, each tool call forwarded to the running app.
pub async fn stdio() -> Result<(), Box<dyn std::error::Error>> {
    let tools: Vec<Tool> = serde_json::from_str(TOOLS_JSON)?;
    let service = Server { tools: Arc::new(tools) }.serve(rmcp::transport::stdio()).await?;
    service.waiting().await?;
    Ok(())
}
