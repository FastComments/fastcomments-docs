FastComments поддържа хостван Model Context Protocol (MCP) сървър, така че AI асистенти и агентни клиенти могат да извикват FastComments API директно. Всеки инструмент, който MCP сървърът излага, се генерира автоматично от публичната OpenAPI спецификация, така че всичко, което REST API може да прави, може да прави и MCP клиент.

Ендпойнтът е stateless и базиран на streamable-HTTP. Няма сесия за поддържане и няма сървърно състояние на клиент.

### Ендпойнт

[inline-code-attrs-start title = 'MCP Ендпойнт'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Свързване с OAuth

Всеки MCP клиент, който поддържа отдалечени сървъри с OAuth (Claude, ChatGPT, Claude Code, Cursor и други), може да се свърже с ендпойнта по-горе без никаква настройка от страната на FastComments. Клиентът се регистрира чрез Dynamic Client Registration или се идентифицира с Client ID Metadata Document, отваря браузър, за да влезете в FastComments и да одобрите достъпа, и получава токен, обвързан с акаунта, в който сте били влезли.

Документите за откриване са на стандартните места:

[inline-code-attrs-start title = 'Откриване'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Вашият потребител трябва да има разрешение API Admin в акаунта, за да одобри връзка. Ако управлявате няколко акаунта, превключете към правилния в дашборда, преди да одобрите.

Клиент може да заяви обхвата `read`, обхвата `write` или и двата. Клиент, който не заявява нищо, получава и двата. Инструменти, които променят данни, не се предлагат на токен само за четене.

Дашбордът има помощник за настройка с готови за поставяне фрагменти. Отворете **Integrate -> MCP Server**, или посетете директно:

[inline-code-attrs-start title = 'Страница за настройка'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Регистрирайте FastComments сървъра с една команда, след което изпълнете `/mcp` в рамките на сесия, за да влезете и да видите наличните инструменти:

[inline-code-attrs-start title = 'Настройка на Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor и други клиенти с конфигурационен файл

Добавете този блок към конфигурацията на MCP сървърите на вашия клиент (`mcp.json` за Cursor). Клиентът отваря браузър за влизане при първото използване.

[inline-code-attrs-start title = 'Конфигурация на MCP клиент'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Отнемане на достъп

Всяка одобрена връзка е изброена под **Integrate -> Connected Apps** в дашборда. Отнемането на една връзка анулира всеки токен, който това приложение притежава. Приложенията се регистрират сами, когато се свързват, и FastComments не ги преглежда, така че отнемете достъпа на всичко, което не разпознавате.

### Използване на токена с REST API

Токенът за достъп, който MCP клиент получава, е обикновено FastComments API удостоверение. Работи на всеки `/api/v1` ендпойнт като bearer токен, така че приложение, свързано чрез MCP, може да извиква и REST API директно:

[inline-code-attrs-start title = 'Bearer токен'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

Тенантът се подразбира от токена. `tenantId` все още може да бъде подаден, но трябва да съвпада. `GET` заявките изискват обхвата `read`, а всичко останало — обхвата `write`.

### Свързване с API ключ

Клиенти, които не могат да завършат влизане през браузър, като например headless сървъри, могат да се удостоверят с API ключ. Подайте `tenantId` и `API_KEY` като query параметри, или като HTTP хедъри `x-tenant-id` и `x-api-key`, ако вашият клиент поддържа персонализирани хедъри:

[inline-code-attrs-start title = 'Ендпойнт с API ключ'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Страницата за настройка генерира този URL за всеки от вашите API ключове.

### Сигурност

URL на ендпойнт, съдържащ API ключ, е тайна: не го поставяйте в публични чатове, екранни снимки или комити. Ако ключ бъде разкрит, сменете го на страницата API Keys в дашборда ви. OAuth токените не носят такъв риск, защото са обвързани с едно приложение и могат да бъдат отнети от Connected Apps.