FastComments — это сервер авторизации OAuth 2.1. Приложение может получить токен, привязанный к одному аккаунту FastComments, и использовать его на любом эндпоинте из этого руководства вместо API-ключа. Именно так приложение Zapier, MCP-сервер и другие сторонние интеграции подключаются к сервису.

Токены выдаются через authorization code flow с PKCE. Client credentials и implicit grant не поддерживаются.

### Discovery

Расположение эндпоинтов, поддерживаемые grant-типы и методы аутентификации публикуются по стандартному metadata URL:

[inline-code-attrs-start title = 'Метаданные сервера авторизации'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Описываемые им эндпоинты:

[inline-code-attrs-start title = 'OAuth эндпоинты'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Аккаунты в регионе EU используют `https://eu.fastcomments.com` в качестве issuer с теми же путями.

### Регистрация клиента

Клиенту необходимы `client_id` и зарегистрированный `redirect_uri`, прежде чем он сможет начать flow. Есть два способа их получить:

- **Dynamic Client Registration.** `POST /oauth/register` с JSON-телом согласно RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). В ответе возвращаются `client_id` и, для конфиденциальных клиентов, `client_secret`. Регистрация не требует аутентификации и ограничена по частоте запросов на IP.
- **Client ID Metadata Document.** Клиент использует `https` URL, которым он владеет, в качестве своего `client_id`. FastComments загружает этот URL и читает из него те же поля метаданных. Вызов регистрации не требуется.

Партнёрские приложения, перечисленные в панели управления FastComments, такие как Zapier, регистрируются FastComments напрямую. Свяжитесь с поддержкой, если вы создаёте листинг для маркетплейса и вам нужен first-party клиент.

### Scopes

[inline-code-attrs-start title = 'Scopes'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

Запрос без указания scope получает оба. Пользователь видит запрошенные scopes на странице согласия. Запрос scope, отличного от этих двух, завершается ошибкой `invalid_scope`.

### Шаг 1 - Запрос авторизации

Направьте браузер пользователя на эндпоинт authorize. PKCE с методом `S256` обязателен для всех клиентов.

[inline-code-attrs-start title = 'Запрос авторизации'; type = 'text'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Параметры запроса авторизации'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthAuthorizeQueryParams {
    response_type: 'code'
    client_id: string
    /** Must exactly match one of the client's registered redirect URIs. **/
    redirect_uri: string
    /** Space separated. Omit to request both scopes. **/
    scope?: 'read' | 'write' | 'read write'
    /** Returned unchanged on the redirect. Use it to bind the callback to the session that started the flow. **/
    state?: string
    /** base64url(sha256(code_verifier)). **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** Optional RFC 8707 resource indicator. If sent, the same value must be sent to the token endpoint. **/
    resource?: string
}
[inline-code-end]

Пользователь при необходимости входит в FastComments и видит страницу согласия, где указаны ваше приложение, аккаунт, к которому оно будет подключено, и запрошенные scopes. Пользователь должен обладать правом **API Admin** на этом аккаунте; все остальные видят ошибку прав доступа вместо формы согласия. Одобрение перенаправляет браузер на ваш `redirect_uri` с параметрами `code` и `state`. Отказ перенаправляет с `error=access_denied`.

Код авторизации действителен 10 минут и может быть обменян один раз. Повторный обмен того же кода отзывает все токены, выданные при первом обмене.

### Шаг 2 - Запрос токена

Обменяйте код на токены. Тело запроса передаётся в формате form encoded. Конфиденциальные клиенты аутентифицируются через `client_secret_basic` (HTTP Basic) или `client_secret_post` (секрет в теле). Публичные клиенты отправляют только `client_id`.

[inline-code-attrs-start title = 'Пример cURL запроса токена'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Тело запроса токена (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestAuthorizationCode {
    grant_type: 'authorization_code'
    client_id: string
    /** Confidential clients only. May be sent as HTTP Basic auth instead. **/
    client_secret?: string
    code: string
    code_verifier: string
    /** Must match the authorization request when sent. **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа с токеном'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** Prefixed fcat_. Valid for one hour. **/
    access_token: string
    token_type: 'bearer'
    /** Seconds until the access token expires. 3600. **/
    expires_in: number
    /** Prefixed fcrt_. Valid for 30 days from issue. **/
    refresh_token: string
    /** Space separated scopes granted. **/
    scope: string
}
[inline-code-end]

Ошибки следуют RFC 6749: JSON-тело с `error` и `error_description`, HTTP 400 для `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` и `unsupported_grant_type`, HTTP 401 для `invalid_client`, HTTP 429 при ограничении частоты запросов.

### Шаг 3 - Вызов API

Отправляйте access token как bearer token. Тенант подразумевается токеном, поэтому `tenantId` необязателен. Если он указан, он должен совпадать с токеном, иначе запрос завершится ошибкой.

[inline-code-attrs-start title = 'Пример cURL с Bearer токеном'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` возвращает тенант, авторизующего пользователя и выданные scopes, что делает его подходящим вызовом для проверки соединения. Запрос с истёкшим или отозванным токеном получает HTTP 401. Запрос, метод которого требует scope, которого нет у токена, получает HTTP 403.

### Шаг 4 - Обновление токенов

[inline-code-attrs-start title = 'Пример cURL запроса обновления'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'Тело запроса токена (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestRefreshToken {
    grant_type: 'refresh_token'
    client_id: string
    client_secret?: string
    refresh_token: string
    /** Optional. Narrows to a subset of the scopes originally granted. **/
    scope?: string
    resource?: string
}
[inline-code-end]

Ответ имеет ту же структуру, что и при обмене кода. Refresh tokens ротируются: каждое обновление возвращает новый `refresh_token` и отзывает старый после 30-секундного льготного окна для параллельных запросов. Предъявление refresh token, ротированного более 30 секунд назад, считается replay-атакой и отзывает весь grant. Партнёрские приложения, зарегистрированные FastComments, освобождены от ротации и получают тот же refresh token со сроком действия, продлённым ещё на 30 дней.

При обновлении также повторно проверяется, что авторизующий пользователь по-прежнему обладает правом API Admin на аккаунте. Если нет, grant отзывается, и ответом будет `invalid_grant`.

### Отзыв токенов

[inline-code-attrs-start title = 'Пример cURL запроса отзыва'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

Отзыв refresh token отзывает все access token, выданные из того же grant. Отзыв access token отзывает только этот токен. Эндпоинт возвращает HTTP 200 с пустым JSON-объектом независимо от того, был ли найден токен, согласно RFC 7009.

Пользователи также могут отозвать подключение через **Connected Apps** в панели управления FastComments. Все токены этого приложения перестают работать немедленно.