FastComments управлява хостван Model Context Protocol (MCP) сървър, така че AI помощници за кодиране и агентни клиенти могат да извикват FastComments API директно. Всяко средство, което MCP сървърът излага, се генерира автоматично от публичния OpenAPI спек, така че всичко, което REST API може да направи, MCP клиент може да направи.

Крайната точка е безсъстояние и базирана на streamable-HTTP. Няма сесия, която да се поддържа жива, няма стъпка за регистрация на клиента и няма сървърно състояние за всеки клиент.

### Крайна точка

[inline-code-attrs-start title = 'Крайна точка на MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Удостоверяването използва същия API ключ като REST API. Можете също така да предадете `tenantId` и ключа като HTTP хедъри `x-tenant-id` и `x-api-key`, ако вашият клиент поддържа потребителски хедъри.

### Предварително попълнена настройка

Таблото за управление има помощник за настройка, който генерира URL и конфигурационни фрагменти, готови за поставяне, за популярни MCP клиенти. Отидете в таблото на вашия акаунт и отворете **Integrate -> MCP Server**, или посетете директно:

[inline-code-attrs-start title = 'Страница за настройка'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

Изберете кой API ключ да използвате от падащото меню, след това копирайте някой от генерираните фрагменти.

### Claude Code

Регистрирайте FastComments сървъра с една команда:

[inline-code-attrs-start title = 'Настройка на Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

След като е регистриран, изпълнете `/mcp` в сесия на Claude Code, за да потвърдите връзката и да изброите наличните инструменти.

### Claude Desktop / Cursor

Добавете този блок към конфигурацията на MCP сървърите на вашия клиент (`claude_desktop_config.json` for Claude Desktop, `mcp.json` for Cursor):

[inline-code-attrs-start title = 'Конфигурация на MCP клиента'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Сигурност

API ключът е вграден в URL. Третирайте URL като тайна: не го поставяйте в публични чатове, скрийншотове или комити. Ако ключът бъде разкрит, сменете го на страницата API Keys в таблото за управление.