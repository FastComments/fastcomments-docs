FastComments запускає розміщений сервер Model Context Protocol (MCP), щоб помічники з кодування на основі ШІ та агентні клієнти могли викликати API FastComments безпосередньо. Кожен інструмент, який сервер MCP надає, автоматично генерується з публічного OpenAPI-специфікації, тому все, що може зробити REST API, може зробити і клієнт MCP.

Кінцева точка безстанна та базується на потоковому HTTP. Немає сесії для підтримки, немає кроку реєстрації клієнта і немає серверного стану для кожного клієнта.

### Кінцева точка

[inline-code-attrs-start title = 'Кінцева точка MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Аутентифікація використовує той самий API-ключ, що й REST API. Ви також можете передати `tenantId` і ключ у вигляді HTTP-заголовків `x-tenant-id` та `x-api-key`, якщо ваш клієнт підтримує користувацькі заголовки.

### Попередньо заповнене налаштування

У панелі керування є помічник налаштування, який генерує URL та готові до вставки фрагменти конфігурації для популярних клієнтів MCP. Перейдіть до панелі керування обліковим записом і відкрийте **Integrate -> MCP Server**, або відвідайте сторінку безпосередньо:

[inline-code-attrs-start title = 'Сторінка налаштування'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

Виберіть ключ API зі спадного списку, потім скопіюйте будь-який із згенерованих фрагментів.

### Claude Code

Зареєструйте сервер FastComments однією командою:

[inline-code-attrs-start title = 'Налаштування Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

Після реєстрації запустіть `/mcp` у сеансі Claude Code, щоб підтвердити з'єднання та перелічити доступні інструменти.

### Claude Desktop / Cursor

Додайте цей блок до конфігурації MCP-серверів вашого клієнта (`claude_desktop_config.json` для Claude Desktop, `mcp.json` для Cursor):

[inline-code-attrs-start title = 'Конфігурація клієнта MCP'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Безпека

API-ключ вбудовано в URL. Ставтеся до цього URL як до секрету: не вставляйте його у публічні чати, скріншоти або коміти. Якщо ключ було скомпрометовано, замініть його на сторінці API Keys у вашій панелі керування.