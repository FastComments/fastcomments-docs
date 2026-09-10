FastComments покреће хостовани Model Context Protocol (MCP) сервер тако да AI асистенти и агентски клијенти могу директно позивати FastComments API. Сваки алат који MCP сервер излаже аутоматски је генерисан из јавне OpenAPI спецификације, тако да све што REST API може, MCP клијент такође може.

Крајња тачка је без стања и заснована на streamable-HTTP. Не постоји сесија која се одржава и нема серверског стања по клијенту.

### Endpoint

[inline-code-attrs-start title = 'MCP крајња тачка'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Повежи се помоћу OAuth

Било који MCP клијент који подржава удаљене сервере са OAuth (Claude, ChatGPT, Claude Code, Cursor и други) може се повезати на горе наведену крајњу тачку без подешавања на страни FastComments. Клијент се региструје преко Dynamic Client Registration или се идентификује помоћу Client ID Metadata Document, отвара прегледач како бисте се пријавили у FastComments и одобрили приступ, и добија токен везан за налог у који сте пријављени.

Документи за откривање се налазе на стандардним локацијама:

[inline-code-attrs-start title = 'Откривање'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Ваш корисник треба да има дозволу API Admin на налогу да би одобрио везу. Ако управљате више налога, пре одобравања пређите на исправан у контролној табли.

Клијент може затражити `read` опсег, `write` опсег, или оба. Клијент који не затражи ништа добија оба. Алати који мењају податке се не нуде токену који има само read дозволу.

Контролна табла има помоћник за подешавање са готовим исечцима за копирање. Отворите **Integrate -> MCP Server**, или га посетите директно:

[inline-code-attrs-start title = 'Страница за подешавање'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

Региструјте FastComments сервер једном командом, а затим покрените `/mcp` унутар сесије да се пријавите и видите доступне алате:

[inline-code-attrs-start title = 'Claude Code подешавање'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor и други клијенти конфигурационог фајла

Додајте овај блок у конфигурацију MCP сервера вашег клијента (`mcp.json` за Cursor). Клијент отвара прегледач за пријаву приликом првог коришћења.

[inline-code-attrs-start title = 'MCP конфигурација клијента'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Одузимање приступа

Свака одобрена веза је наведена под **Integrate -> Connected Apps** у контролној табли. Одузимање једне везе поништава сваки токен који та апликација има. Апликације се региструју када се повежу, а FastComments их не прегледа, па одузмите све што не препознајете.

### Коришћење токена са REST API-јем

Приступни токен који MCP клијент добије је уобичајена FastComments API акредитив. Ради на свакој `/api/v1` крајњој тачки као bearer токен, тако да апликација која се повезала преко MCP-а такође може директно позивати REST API:

[inline-code-attrs-start title = 'Bearer токен'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

Тенант је подразумеван токеном. `tenantId` се и даље може проследити, али мора да се поклапа. `GET` захтеви захтевају `read` опсег, а све остало захтева `write` опсег.

### Повежи се помоћу API кључа

Клијенти који не могу да заврше пријаву у прегледачу, као што су безглави сервери, могу се аутентификоваћим помоћу API кључа. Проследите `tenantId` и `API_KEY` као параметре упита, или као `x-tenant-id` и `x-api-key` HTTP заглавља ако ваш клијент подржава прилагођена заглавља:

[inline-code-attrs-start title = 'API кључ крајња тачка'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

Страница за подешавање генерише овај URL за сваки ваш API кључ.

### Безбедност

URL крајње тачке који садржи API кључ је тајна: немојте га копирати у јавне разговоре, снимке екрана или комите. Ако кључ буде изложен, ротирајте га на страници API кључева у вашој контролној табли. OAuth токени не представљају такву ризику јер су везани за једну апликацију и могу се одузети из Connected Apps.