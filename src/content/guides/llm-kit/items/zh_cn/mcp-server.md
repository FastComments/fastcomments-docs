FastComments 运行一个托管的模型上下文协议 (MCP) 服务器，因此 AI 编码助手和具代理能力的客户端可以直接调用 FastComments API。MCP 服务器公开的每一个工具都是根据公开的 OpenAPI 规范自动生成的，所以 REST API 能做的任何事情，MCP 客户端都能做。

该端点是无状态的、基于可流式 HTTP。无需维护会话、无需客户端注册步骤，并且服务器不会为每个客户端保存状态。

### 端点

[inline-code-attrs-start title = 'MCP 端点'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

身份验证使用与 REST API 相同的 API 密钥。如果您的客户端支持自定义头，也可以将 `tenantId` 和密钥作为 `x-tenant-id` 和 `x-api-key` HTTP 头传递。

### 预置设置

仪表板提供一个设置助手，可为流行的 MCP 客户端生成 URL 和可直接粘贴的配置片段。转到您的账户仪表板并打开 **集成 -> MCP 服务器**，或直接访问：

[inline-code-attrs-start title = '设置页面'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

从下拉菜单中选择要使用的 API 密钥，然后复制任何生成的片段。

### Claude Code

通过一条命令注册 FastComments 服务器：

[inline-code-attrs-start title = 'Claude Code 设置'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

注册后，在 Claude Code 会话中运行 `/mcp` 以确认连接并列出可用工具。

### Claude Desktop / Cursor

将此块添加到您客户端的 MCP servers 配置中（Claude Desktop 为 `claude_desktop_config.json`，Cursor 为 `mcp.json`）：

[inline-code-attrs-start title = 'MCP 客户端配置'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "mcpServers": {
    "fastcomments": {
      "type": "http",
      "url": "https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY"
    }
  }
}
[inline-code-end]

### 安全

API 密钥嵌入在 URL 中。请将该 URL 视为机密：不要将其粘贴到公开聊天、截图或提交中。如果密钥被泄露，请在仪表板的 API 密钥 页面上对其进行轮换。