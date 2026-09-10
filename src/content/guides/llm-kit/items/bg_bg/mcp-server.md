FastComments управлява хостван сървър за Model Context Protocol (MCP), така че AI асистентите и агентните клиенти да могат директно да извикват FastComments API. Всеки инструмент, който сървърът MCP предоставя, е автоматично генериран от публичната OpenAPI спецификация, така че всичко, което REST API може да направи, клиентът MCP също може да направи.

Крайната точка е без състояние и базирана на streamable-HTTP. Няма сесия, която да се поддържа, и няма сървърно състояние за всеки клиент.

### Endpoint

[inline-code-attrs-start title = 'MCP Крайна точка'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Connect with OAuth

Всеки MCP клиент, който поддържа отдалечени сървъри с OAuth (Claude, ChatGPT, Claude Code, Cursor и други), може да се свърже с горната крайна точка без настройка от страна на FastComments. Клиентът се регистрира чрез Dynamic Client Registration или се идентифицира с документ за метаданни на клиентския идентификатор, отваря браузър, за да влезете във FastComments и одобрите достъпа, и получава токен, свързан с акаунта, в който сте влезли.

Документите за откриване се намират на стандартните места:

[inline-code-attrs-start title = 'Откриване'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Вашият потребител се нуждае от разрешение API Admin в акаунта, за да одобри връзка. Ако управлявате няколко акаунта, превключете към правилния в таблото преди одобряването.

Клиентът може да заяви обхвата `read`, обхвата `write` или и двата. Клиент, който не заяви нищо, получава и двете. Инструментите, които променят данни, не се предлагат на токен само за четене.

Таблото има помощник за настройка с готови за поставяне фрагменти. Отворете **Integrate -> MCP Server**, или посетете директно:

[inline-code-attrs-start title = 'Страница за настройка'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Регистрирайте FastComments сървъра с една команда, след което изпълнете `/mcp` в сесия, за да влезете и да изброите наличните инструменти:

[inline-code-attrs-start title = 'Claude Code Настройка'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor and other config-file clients

Добавете този блок към конфигурацията на MCP сървърите на вашия клиент (`mcp.json` за Cursor). Клиентът отваря браузър, за да влезе при първото използване.

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

### Revoking access

Всяка одобрена връзка е изброена под **Integrate -> Connected Apps** в таблото. Прекратяването на една от тях анулира всеки токен, който приложението притежава. Приложенията се регистрират, когато се свържат, и FastComments не ги преглежда, затова прекратете всичко, което не разпознавате.

### Using the token with the REST API

Достъпният токен, който MCP клиент получава, е обикновено FastComments API удостоверение. Той работи на всяка крайна точка `/api/v1` като токен за носител, така че приложение, свързано чрез MCP, може също директно да извика REST API:

[inline-code-attrs-start title = 'Токен за носител'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

Наемателят (tenant) се подразбира от токена. `tenantId` все още може да се предаде, но трябва да съвпада. `GET` заявките изискват обхвата `read`, а всичко останало изисква обхвата `write`.

### Connect with an API key

Клиенти, които не могат да завършат вход чрез браузър, като безглава сървъри, могат вместо това да се удостоверяват с API ключ. Предайте `tenantId` и `API_KEY` като параметри на заявката, или като HTTP заглавия `x-tenant-id` и `x-api-key`, ако клиентът ви поддържа персонализирани заглавия:

[inline-code-attrs-start title = 'Крайна точка за API ключ'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Страницата за настройка генерира този URL за всеки от вашите API ключове.

### Security

URL на крайна точка, който съдържа API ключ, е тайна: не го поставяйте в публични чатове, скрийншоти или комити. Ако ключът бъде изложен, ротирате го на страницата API Keys в таблото си. OAuth токените не носят такъв риск, тъй като са свързани с едно приложение и могат да бъдат анулирани от Connected Apps.