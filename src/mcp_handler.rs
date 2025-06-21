use rust_mcp_sdk::schema::{
    schema_utils::CallToolError, CallToolRequest, CallToolResult, ListToolsRequest,
    ListToolsResult, RpcError,
};
use rust_mcp_sdk::{mcp_server::ServerHandler, McpServer};

use crate::tools::{Human, HumanTools};

pub struct Handler<H> {
    human: H,
}

impl<H: Human> Handler<H> {
    pub fn new(human: H) -> Self {
        Self { human }
    }
}

#[async_trait::async_trait]
#[allow(unused)]
impl<H: Human> ServerHandler for Handler<H> {
    async fn handle_list_tools_request(
        &self,
        request: ListToolsRequest,
        runtime: &dyn McpServer,
    ) -> Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            meta: None,
            next_cursor: None,
            tools: HumanTools::tools(),
        })
    }

    async fn handle_call_tool_request(
        &self,
        request: CallToolRequest,
        runtime: &dyn McpServer,
    ) -> Result<CallToolResult, CallToolError> {
        let tool_params: HumanTools =
            HumanTools::try_from(request.params).map_err(CallToolError::new)?;

        match tool_params {
            HumanTools::AskHumanTool(ask_human_tool) => ask_human_tool.call_tool(&self.human).await,
        }
    }
}
