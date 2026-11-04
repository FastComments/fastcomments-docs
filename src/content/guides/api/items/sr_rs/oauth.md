FastComments је OAuth 2.1 сервер за ауторизацију. Апликација може добити токен који је везан за један FastComments налог и користити га на сваком крајњем тачки у овом водичу уместо API кључа. На овај начин се Zapier апликација, MCP сервер и друге интеграције трећих страна повезују.

Токени се издају путем authorization code тока са PKCE. Не постоји client credentials нити implicit grant.

### Откривање

Локације крајњих тачака, подржани грантови и методе аутентификације објављени су на стандардној metadata URL адреси:

[inline-code-attrs-start title = 'Metadata сервера за ауторизацију'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Крајње тачке које описује:

[inline-code-attrs-start title = 'OAuth крајње тачке'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Налози у EU региону користе `https://eu.fastcomments.com` као issuer, са истим путањама.

### Регистрација клијента

Клијенту су потребни `client_id` и регистровани `redirect_uri` пре него што може покренути ток. Постоје два начина да их добије:

- **Динамичка регистрација клијента.** `POST /oauth/register` са JSON телом према RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). Одговор садржи `client_id` и, за поверљиве клијенте, `client_secret`. Регистрација је неаутентификована и ограничена по IP адреси.
- **Client ID Metadata Document.** Клијент користи `https` URL који контролише као свој `client_id`. FastComments преузима тај URL и чита иста metadata поља из њега. Није потребан позив за регистрацију.

Партнерске апликације наведене у FastComments контролном панелу, као што је Zapier, региструје директно FastComments. Контактирајте подршку ако градите marketplace листинг и треба вам first-party клијент.

### Scopes

[inline-code-attrs-start title = 'Scopes'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

Захтев који не тражи ниједан scope добија оба. Корисник види тражене scopes на страници за сагласност. Захтев за scope који није један од ова два неуспева са `invalid_scope`.

### Корак 1 - Захтев за ауторизацију

Пошаљите прегледач корисника на authorize крајњу тачку. PKCE са методом `S256` је обавезан за све клијенте.

[inline-code-attrs-start title = 'Захтев за ауторизацију'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET https://fastcomments.com/oauth/authorize
    ?response_type=code
    &client_id=YOUR_CLIENT_ID
    &redirect_uri=https://example.com/oauth/callback
    &scope=read%20write
    &state=RANDOM_STATE
    &code_challenge=BASE64URL_SHA256_OF_VERIFIER
    &code_challenge_method=S256
[inline-code-end]

[inline-code-attrs-start title = 'Параметри захтева за ауторизацију'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthAuthorizeQueryParams {
    response_type: 'code'
    client_id: string
    /** Мора тачно одговарати једном од регистрованих redirect URI адреса клијента. **/
    redirect_uri: string
    /** Раздвојени размаком. Изоставите да тражите оба scope-а. **/
    scope?: 'read' | 'write' | 'read write'
    /** Враћа се непромењен при преусмерењу. Користите га да вежете callback за сесију која је покренула ток. **/
    state?: string
    /** base64url(sha256(code_verifier)). **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** Опциони RFC 8707 resource индикатор. Ако се пошаље, иста вредност мора бити послата на token крајњу тачку. **/
    resource?: string
}
[inline-code-end]

Корисник се пријављује на FastComments ако је потребно и види страницу за сагласност која наводи вашу апликацију, налог на који ће бити повезана и тражене scopes. Корисник мора имати дозволу **API Admin** на том налогу; свако други види грешку о дозволама уместо обрасца за сагласност. Одобравање преусмерава прегледач на вашу `redirect_uri` са `code` и `state`. Одбијање преусмерава са `error=access_denied`.

Authorization code важи 10 минута и може се заменити само једном. Друга замена истог кода опозива све токене које је прва замена произвела.

### Корак 2 - Захтев за токен

Замените код за токене. Тело је form encoded. Поверљиви клијенти се аутентификују са `client_secret_basic` (HTTP Basic) или `client_secret_post` (тајна у телу). Јавни клијенти шаљу само `client_id`.

[inline-code-attrs-start title = 'cURL пример захтева за токен'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=authorization_code' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'code=fcac_...' \
  --data 'code_verifier=YOUR_PKCE_VERIFIER' \
  --data 'redirect_uri=https://example.com/oauth/callback'
[inline-code-end]

[inline-code-attrs-start title = 'Тело захтева за токен (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestAuthorizationCode {
    grant_type: 'authorization_code'
    client_id: string
    /** Само поверљиви клијенти. Може бити послато као HTTP Basic auth уместо тога. **/
    client_secret?: string
    code: string
    code_verifier: string
    /** Мора одговарати захтеву за ауторизацију када се шаље. **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора са токеном'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** Префикс fcat_. Важи један сат. **/
    access_token: string
    token_type: 'bearer'
    /** Секунди до истека access токена. 3600. **/
    expires_in: number
    /** Префикс fcrt_. Важи 30 дана од издавања. **/
    refresh_token: string
    /** Раздвојени размаком додељени scopes. **/
    scope: string
}
[inline-code-end]

Грешке прате RFC 6749: JSON тело са `error` и `error_description`, HTTP 400 за `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` и `unsupported_grant_type`, HTTP 401 за `invalid_client`, HTTP 429 када је ограничење захтева активано.

### Корак 3 - Позивање API-ја

Пошаљите access токен као bearer токен. Тенант је имплициран токеном, тако да је `tenantId` опциони. Ако је дат, мора одговарати токену или захтев неуспева.

[inline-code-attrs-start title = 'cURL пример са Bearer токеном'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` враћа тенант, ауторизујућег корисника и додељене scopes, што га чини правим позивом за тестирање везе. Захтев са истеклим или опозваним токеном добија HTTP 401. Захтев чија метода захтева scope који токен не поседује добија HTTP 403.

### Корак 4 - Освежавање

[inline-code-attrs-start title = 'cURL пример захтева за освежавање'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'Тело захтева за токен (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestRefreshToken {
    grant_type: 'refresh_token'
    client_id: string
    client_secret?: string
    refresh_token: string
    /** Опциони. Сужава на подскуп првобитно додељених scope-ова. **/
    scope?: string
    resource?: string
}
[inline-code-end]

Одговор има исти облик као замена кода. Refresh токени ротирају: свако освежавање враћа нови `refresh_token` и опозива стари након 30-секундног периода милости за конкурентне захтеве. Приказивање refresh токена који је ротиран пре више од 30 секунди третира се као replay и опозива цео грант. Партнерске апликације регистроване од стране FastComments изузете су од ротације и добијају назад исти refresh токен са роком трајања продуженим за још 30 дана.

Освежавање такође поново проверава да ли ауторизујући корисник још увек има API Admin на налогу. Ако нема, грант се опозива и одговор је `invalid_grant`.

### Опозив

[inline-code-attrs-start title = 'cURL пример захтева за опозив'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

Опозив refresh токена опозива све access токене издате из истог гранта. Опозив access токена опозива само тај токен. Крајња тачка враћа HTTP 200 са празним JSON објектом без обзира да ли је токен пронађен, према RFC 7009.

Корисници такође могу опозвати везу из **Connected Apps** у FastComments контролном панелу. Сви токени за ту апликацију престају да раде одмах.