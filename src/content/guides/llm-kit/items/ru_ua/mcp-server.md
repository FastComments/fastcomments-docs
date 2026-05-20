---
FastComments запускает размещённый сервер Model Context Protocol (MCP), чтобы AI-ассистенты для кодирования и агентні клієнти могли вызывать API FastComments напрямую. Все инструменты, которые предоставляет сервер MCP, автоматически генерируются из публичной спецификации OpenAPI, так что всё, что может сделать REST API, может сделать и клиент MCP.

Конечная точка не хранит состояние и основана на потоковом HTTP. Нет сессии, которую нужно поддерживать, нет шага регистрации клиента и нет серверного состояния для отдельных клиентов.

### Конечная точка

[inline-code-attrs-start title = 'Конечная точка MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Аутентификация использует тот же API-ключ, что и REST API. Вы также можете передать `tenantId` и ключ в HTTP-заголовках `x-tenant-id` и `x-api-key`, если ваш клиент поддерживает пользовательские заголовки.

### Предзаполненная настройка

В панели управления есть помощник настройки, который генерирует URL и готовые для вставки фрагменты конфигурации для популярных клиентов MCP. Перейдите в панель управления аккаунтом и откройте **Integrate -> MCP Server**, либо посетите напрямую:

[inline-code-attrs-start title = 'Страница настройки'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

Выберите API-ключ для использования в выпадающем списке, затем скопируйте любой из сгенерированных фрагментов.

### Claude Code

Зарегистрируйте сервер FastComments одной командой:

[inline-code-attrs-start title = 'Настройка Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

После регистрации выполните `/mcp` в сессии Claude Code, чтобы подтвердить подключение и увидеть список доступных инструментов.

### Claude Desktop / Cursor

Добавьте этот блок в конфигурацию серверов MCP вашего клиента (`claude_desktop_config.json` для Claude Desktop, `mcp.json` для Cursor):

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

### Безопасность

API-ключ встроен в URL. Обращайтесь с URL как с секретом: не вставляйте его в публичные чаты, скриншоты или коммиты. Если ключ был скомпрометирован, поменяйте его на странице API Keys в вашей панели управления.

---