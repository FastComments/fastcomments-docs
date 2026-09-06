FastComments runs a hosted Model Context Protocol (MCP) server so AI assistants and agentic clients can call the FastComments API directly. Every tool the MCP server exposes is auto-generated from the public OpenAPI spec, so anything the REST API can do, an MCP client can do.

The endpoint is stateless and streamable-HTTP based. There is no session to keep alive and no server-side state per client.

### Endpoint

[inline-code-attrs-start title = 'MCP Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Connect with OAuth

Any MCP client that supports remote servers with OAuth (Claude, ChatGPT, Claude Code, Cursor, and others) can connect to the endpoint above with no setup on the FastComments side. The client registers itself through Dynamic Client Registration or identifies itself with a Client ID Metadata Document, opens a browser so you can sign in to FastComments and approve access, and receives a token bound to the account you were signed in to.

The discovery documents are at the standard locations:

[inline-code-attrs-start title = 'Discovery'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Your user needs the API Admin permission on the account to approve a connection. If you manage several accounts, switch to the right one in the dashboard before approving.

A client can request the `read` scope, the `write` scope, or both. A client that requests nothing gets both. Tools that change data are not offered to a read-only token.

The dashboard has a setup helper with ready-to-paste snippets. Open **Integrate -> MCP Server**, or visit it directly:

[inline-code-attrs-start title = 'Setup Page'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Register the FastComments server with one command, then run `/mcp` inside a session to sign in and list the available tools:

[inline-code-attrs-start title = 'Claude Code Setup'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor and other config-file clients

Add this block to your client's MCP servers config (`mcp.json` for Cursor). The client opens a browser to sign in on first use.

[inline-code-attrs-start title = 'MCP Client Config'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

Every approved connection is listed under **Integrate -> Connected Apps** in the dashboard. Revoking one invalidates every token that application holds. Applications register themselves when they connect and FastComments does not review them, so revoke anything you do not recognize.

### Using the token with the REST API

The access token an MCP client obtains is a regular FastComments API credential. It works on every `/api/v1` endpoint as a bearer token, so an application that connected through MCP can also call the REST API directly:

[inline-code-attrs-start title = 'Bearer Token'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

The tenant is implied by the token. A `tenantId` may still be passed but it must match. `GET` requests need the `read` scope and everything else needs the `write` scope.

### Connect with an API key

Clients that cannot complete a browser sign-in, such as headless servers, can authenticate with an API key instead. Pass `tenantId` and `API_KEY` as query parameters, or as the `x-tenant-id` and `x-api-key` HTTP headers if your client supports custom headers:

[inline-code-attrs-start title = 'API Key Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

The setup page generates this URL for each of your API keys.

### Security

An endpoint URL that contains an API key is a secret: do not paste it into public chats, screenshots, or commits. If a key is exposed, rotate it on the API Keys page in your dashboard. OAuth tokens carry no such risk because they are bound to one application and can be revoked from Connected Apps.
