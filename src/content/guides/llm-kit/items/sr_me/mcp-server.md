FastComments покреће хостовани Model Context Protocol (MCP) сервер тако да AI асистенти за кодирање и агентски клијенти могу директно да позивају FastComments API. Сваки алат који MCP сервер издваја је аутоматски генерисан из јавне OpenAPI спецификације, па све што REST API може да уради, MCP клијент може да уради.

Крајња тачка је без стања и базирана на streamable-HTTP. Нема сесије коју треба одржавати, нема корака регистрације клијента и нема стања на серверу по клијенту.

### Endpoint

[inline-code-attrs-start title = 'MCP крајња тачка'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Аутентификација користи исти API кључ као и REST API. Такође можете проследити `tenantId` и кључ као `x-tenant-id` и `x-api-key` HTTP заглавља ако ваш клијент подржава прилагођена заглавља.

### Унапред попуњено подешавање

Контролна табла има помоћник за подешавање који генерише URL и готове исечке конфигурације спремне за лепљење за популарне MCP клијенте. Идите на контролну таблу налога и отворите **Интеграције -> MCP Server**, или посетите директно:

[inline-code-attrs-start title = 'Страница за подешавање'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

Изаберите који API кључ желите да користите из падајућег менија, затим копирајте било који од генерисаних исечака.

### Claude Code

Региструјте FastComments сервер једном командом:

[inline-code-attrs-start title = 'Подешавање Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments 'https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY'
[inline-code-end]

Након регистрације, покрените `/mcp` у оквиру Claude Code сесије да бисте потврдили везу и видели листу расположивих алата.

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

API кључ је уграђен у URL. Понашајте се према URL-у као према тајни: не лепите га у јавне разговоре, снимке екрана или комитове. Ако је кључ откривен, ротирајте га на страници API Keys у вашој контролној табли.

---