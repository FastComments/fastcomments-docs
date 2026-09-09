FastComments 运行一个托管的模型上下文协议（MCP）服务器，以便 AI 助手和代理客户端可以直接调用 FastComments API。MCP 服务器公开的每个工具都是从公共 OpenAPI 规范自动生成的，因此 REST API 能做的，MCP 客户端也能做。

该端点是无状态且基于可流式的 HTTP。没有需要保持的会话，也没有每个客户端的服务器端状态。

### Endpoint

[inline-code-attrs-start title = 'MCP 端点'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Connect with OAuth

任何支持 OAuth 远程服务器的 MCP 客户端（Claude、ChatGPT、Claude Code、Cursor 等）都可以无需在 FastComments 端进行设置即可连接上述端点。客户端通过动态客户端注册或使用客户端 ID 元数据文档进行注册，打开浏览器让您登录 FastComments 并批准访问，然后收到绑定到您登录账户的令牌。

发现文档位于标准位置：

[inline-code-attrs-start title = '发现'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

您的用户需要在账户上拥有 API 管理员权限才能批准连接。如果您管理多个账户，请在批准前在仪表板中切换到正确的账户。

客户端可以请求 `read` 范围、`write` 范围或两者兼有。未请求任何范围的客户端将获得两者。更改数据的工具不会提供给只读令牌。

仪表板提供了带有可直接粘贴代码段的设置助手。打开 **Integrate -> MCP Server**，或直接访问：

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

### Cursor and other config-file clients

将此块添加到客户端的 MCP 服务器配置中（Cursor 使用 `mcp.json`）。客户端在首次使用时会打开浏览器进行登录。

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

### Revoking access

每个已批准的连接都会在仪表板的 **Integrate -> Connected Apps** 中列出。撤销其中一个会使该应用持有的所有令牌失效。应用在连接时自行注册，FastComments 不会审查它们，因此请撤销任何您不认识的应用。

### Using the token with the REST API

MCP 客户端获取的访问令牌是普通的 FastComments API 凭证。它可以作为 Bearer 令牌在每个 `/api/v1` 端点上使用，因此通过 MCP 连接的应用也可以直接调用 REST API：

[inline-code-attrs-start title = 'Bearer 令牌'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

租户由令牌隐含。仍然可以传递 `tenantId`，但必须匹配。`GET` 请求需要 `read` 范围，其他请求需要 `write` 范围。

### Connect with an API key

无法完成浏览器登录的客户端（例如无头服务器），可以改用 API 密钥进行身份验证。将 `tenantId` 和 `API_KEY` 作为查询参数传递，或在客户端支持自定义头时使用 `x-tenant-id` 和 `x-api-key` HTTP 头：

[inline-code-attrs-start title = 'API 密钥端点'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

设置页面会为您的每个 API 密钥生成此 URL。

### Security

包含 API 密钥的端点 URL 是机密信息：不要将其粘贴到公开聊天、截图或提交中。如果密钥泄露，请在仪表板的 API 密钥页面上进行轮换。OAuth 令牌不存在此类风险，因为它们绑定到单个应用，并且可以在 Connected Apps 中撤销。