FastComments покреће хостовани Model Context Protocol (MCP) сервер, тако да AI асистенти за кодирање и агентски клијенти могу директно да позивају FastComments API. Сви алати које MCP сервер излаже аутоматски су генерисани из јавне OpenAPI спецификације, па све што REST API може да уради, може и MCP клијент.

Крајња тачка је без стања и заснована на streamable-HTTP-у. Нема сесије коју треба одржавати, нема корака регистрације клијента и нема стања на серверу по клијенту.

### Endpoint

[inline-code-attrs-start title = 'MCP крајња тачка'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Аутентикација користи исти API кључ као REST API. Такође можете проследити `tenantId` и кључ као HTTP хедере `x-tenant-id` и `x-api-key` ако ваш клијент подржава прилагођене хедере.

### Pre-filled setup

Контролна табла има помоћник за подешавање који генерише URL и спремне за лепљење конфигурационе исечке за популарне MCP клијенте. Идите на контролну таблу свог налога и отворите **Integrate -> MCP Server**, или је посетите директно:

[inline-code-attrs-start title = 'Страница за подешавање'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

Изаберите који API кључ желите да користите из падајућег менија, затим копирајте било који од генерисаних исечака.

### Claude Code

Региструјте FastComments сервер са једном командом:

[inline-code-attrs-start title = 'Подешавање Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

Након што је регистрован, покрените `/mcp` унутар Claude Code сесије да потврдите везу и прикажете доступне алате.

### Claude Desktop / Cursor

Додајте овај блок у конфигурацију MCP сервера вашег клијента (`claude_desktop_config.json` за Claude Desktop, `mcp.json` за Cursor):

[inline-code-attrs-start title = 'Конфигурација MCP клијента'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

API кључ је уграђен у URL. Поступајте са URL-ом као са тајном: немојте га налепљивати у јавне четове, снимке екрана или комите. Ако је кључ откривен, замените га на страници API кључева у вашој контролној табли.