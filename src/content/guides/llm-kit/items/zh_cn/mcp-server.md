FastComments 运行一个托管的模型上下文协议（MCP）服务器，使 AI 助手和智能体客户端可以直接调用 FastComments API。MCP 服务器暴露的每个工具都是从公开的 OpenAPI 规范自动生成的，因此 REST API 能做的任何事情，MCP 客户端也能做。

该端点是无状态的，基于可流式传输的 HTTP。没有需要保持的会话，也没有每个客户端的服务器端状态。

### 端点

[inline-code-attrs-start title = 'MCP 端点'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### 通过 OAuth 连接

任何支持通过 OAuth 连接远程服务器的 MCP 客户端（Claude、ChatGPT、Claude Code、Cursor 等）都可以连接到上述端点，无需在 FastComments 侧进行任何设置。客户端通过动态客户端注册（Dynamic Client Registration）自行注册，或通过客户端 ID 元数据文档（Client ID Metadata Document）标识自己，然后打开浏览器让你登录 FastComments 并批准访问，最后获得一个与你登录账户绑定的令牌。

发现文档位于标准位置：

[inline-code-attrs-start title = '发现文档'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

你的用户需要账户上的 API Admin 权限才能批准连接。如果你管理多个账户，请在批准前先在仪表板中切换到正确的账户。

客户端可以请求 `read` 作用域、`write` 作用域，或两者都请求。未请求任何作用域的客户端将同时获得两者。更改数据的工具不会提供给只读令牌。

仪表板中有一个设置助手，提供可直接粘贴的代码片段。打开 **Integrate -> MCP Server**，或直接访问：

[inline-code-attrs-start title = '设置页面'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

使用一条命令注册 FastComments 服务器，然后在会话中运行 `/mcp` 进行登录并列出可用工具：

[inline-code-attrs-start title = 'Claude Code 设置'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor 及其他基于配置文件的客户端

将以下代码块添加到客户端的 MCP 服务器配置中（Cursor 为 `mcp.json`）。客户端在首次使用时会打开浏览器进行登录。

[inline-code-attrs-start title = 'MCP 客户端配置'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "mcpServers": {
    "fastcomments": {
      "type": "http",
      "url": "https://fastcomments.com/mcp"
    }
  }
}
[inline-code-end]

### 撤销访问

每个已批准的连接都会列在仪表板的 **Integrate -> Connected Apps** 下。撤销某个连接会使该应用持有的所有令牌失效。应用在连接时会自行注册，FastComments 不会对它们进行审核，因此请撤销任何你不认识的应用。

### 将令牌用于 REST API

MCP 客户端获取的访问令牌是一个普通的 FastComments API 凭据。它可以作为 bearer 令牌用于所有 `/api/v1` 端点，因此通过 MCP 连接的应用也可以直接调用 REST API：

[inline-code-attrs-start title = 'Bearer 令牌'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

租户由令牌隐含指定。仍然可以传入 `tenantId`，但它必须匹配。`GET` 请求需要 `read` 作用域，其他所有请求需要 `write` 作用域。

### 使用 API 密钥连接

无法完成浏览器登录的客户端（例如无头服务器）可以改用 API 密钥进行身份验证。将 `tenantId` 和 `API_KEY` 作为查询参数传递，或者如果你的客户端支持自定义 HTTP 头，则作为 `x-tenant-id` 和 `x-api-key` HTTP 头传递：

[inline-code-attrs-start title = 'API 密钥端点'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

设置页面会为你的每个 API 密钥生成此 URL。

### 安全性

包含 API 密钥的端点 URL 属于机密信息：不要将其粘贴到公开聊天、截图或提交记录中。如果密钥泄露，请在仪表板的 API Keys 页面上轮换该密钥。OAuth 令牌没有此类风险，因为它们绑定到单个应用，并且可以从 Connected Apps 中撤销。