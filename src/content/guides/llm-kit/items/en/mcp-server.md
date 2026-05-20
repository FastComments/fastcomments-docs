FastComments runs a hosted Model Context Protocol (MCP) server so AI coding assistants and agentic clients can call the FastComments API directly. Every tool the MCP server exposes is auto-generated from the public OpenAPI spec, so anything the REST API can do, an MCP client can do.

The endpoint is stateless and streamable-HTTP based. There's no session to keep alive, no client registration step, and no server-side state per client.

### Endpoint

[inline-code-attrs-start title = 'MCP Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Authentication uses the same API key as the REST API. You can also pass `tenantId` and the key as `x-tenant-id` and `x-api-key` HTTP headers if your client supports custom headers.

### Pre-filled setup

The dashboard has a setup helper that generates the URL and ready-to-paste config snippets for popular MCP clients. Go to your account dashboard and open **Integrate -> MCP Server**, or visit it directly:

[inline-code-attrs-start title = 'Setup Page'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

Pick which API key to use from the dropdown, then copy any of the generated snippets.

### Claude Code

Register the FastComments server with one command:

[inline-code-attrs-start title = 'Claude Code Setup'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

After it's registered, run `/mcp` inside a Claude Code session to confirm the connection and list available tools.

### Claude Desktop / Cursor

Add this block to your client's MCP servers config (`claude_desktop_config.json` for Claude Desktop, `mcp.json` for Cursor):

[inline-code-attrs-start title = 'MCP Client Config'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Security

The API key is embedded in the URL. Treat the URL like a secret: do not paste it into public chats, screenshots, or commits. If a key is exposed, rotate it on the API Keys page in your dashboard.
