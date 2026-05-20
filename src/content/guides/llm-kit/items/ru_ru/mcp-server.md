FastComments запускает размещённый сервер Model Context Protocol (MCP), чтобы помощники по кодированию на базе ИИ и агентные клиенты могли вызывать FastComments API напрямую. Каждый инструмент, который сервер MCP предоставляет, автоматически генерируется из публичной спецификации OpenAPI, поэтому всё, что может сделать REST API, может сделать и клиент MCP.

The endpoint is stateless and streamable-HTTP based. There's no session to keep alive, no client registration step, and no server-side state per client.

### Endpoint

[inline-code-attrs-start title = 'Конечная точка MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Аутентификация использует тот же API-ключ, что и REST API. Вы также можете передать `tenantId` и ключ в HTTP-заголовках `x-tenant-id` и `x-api-key`, если ваш клиент поддерживает пользовательские заголовки.

### Pre-filled setup

В панели управления есть помощник настройки, который генерирует URL и готовые к вставке фрагменты конфигурации для популярных MCP клиентов. Перейдите в панель управления аккаунтом и откройте **Integrate -> MCP Server**, или посетите напрямую:

[inline-code-attrs-start title = 'Страница настройки'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

Выберите, какой API-ключ использовать, из выпадающего списка, затем скопируйте любой из сгенерированных фрагментов.

### Claude Code

Register the FastComments server with one command:

[inline-code-attrs-start title = 'Настройка Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

После регистрации выполните `/mcp` в сессии Claude Code, чтобы подтвердить соединение и перечислить доступные инструменты.

### Claude Desktop / Cursor

Add this block to your client's MCP servers config (`claude_desktop_config.json` for Claude Desktop, `mcp.json` for Cursor):

[inline-code-attrs-start title = 'Конфигурация клиента MCP'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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