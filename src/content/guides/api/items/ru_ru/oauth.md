FastComments — это сервер авторизации OAuth 2.1. Приложение может получить токен, привязанный к одной учётной записи FastComments, и использовать его на каждом эндпоинте в этом руководстве вместо API‑ключа. Именно так приложение Zapier, сервер MCP и другие сторонние интеграции подключаются.

Токены выдаются через поток кода авторизации с PKCE. Клиентские учётные данные или неявный грант не поддерживаются.

### Обнаружение

Endpoint locations, supported grants, and auth methods are published at the standard metadata URL:

[inline-code-attrs-start title = 'Метаданные сервера авторизации'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

Эндпоинты, которые он описывает:

[inline-code-attrs-start title = 'Точки доступа OAuth'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Учётные записи в регионе ЕС используют `https://eu.fastcomments.com` в качестве издателя, с теми же путями.

### Регистрация клиента

Клиенту нужен `client_id` и зарегистрированный `redirect_uri`, прежде чем он сможет начать поток. Существует два способа их получить:

- **Динамическая регистрация клиента.** `POST /oauth/register` с JSON‑тело согласно RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). Ответ содержит `client_id` и, для конфиденциальных клиентов, `client_secret`. Регистрация не требует аутентификации и ограничена по частоте запросов для каждого IP.
- **Документ метаданных Client ID.** Клиент использует контролируемый им `https`‑URL в качестве `client_id`. FastComments получает этот URL и читает из него те же поля метаданных. Вызов регистрации не требуется.

Партнёрские приложения, перечисленные в панели FastComments, такие как Zapier, регистрируются FastComments напрямую. Обратитесь в поддержку, если вы создаёте листинг в маркетплейсе и вам нужен клиент первой стороны.

### Области доступа

[inline-code-attrs-start title = 'Области доступа'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

Запрос без указания области доступа получает обе. Пользователь видит запрошенные области доступа на странице согласия. Запрос на область, отличную от этих двух, завершается ошибкой `invalid_scope`.

### Шаг 1 — Запрос авторизации

Перенаправьте браузер пользователя на эндпоинт авторизации. PKCE с методом `S256` требуется для каждого клиента.

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
    /** Должен точно соответствовать одному из зарегистрированных URI перенаправления клиента. **/
    redirect_uri: string
    /** Разделены пробелом. Опустите, чтобы запросить обе области доступа. **/
    scope?: 'read' | 'write' | 'read write'
    /** Возвращается без изменений при перенаправлении. Используется для привязки обратного вызова к сессии, которая запустила поток. **/
    state?: string
    /** base64url(sha256(code_verifier)). **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** Необязательный индикатор ресурса RFC 8707. Если отправлен, то то же значение должно быть отправлено в эндпоинт токена. **/
    resource?: string
}
[inline-code-end]

Пользователь при необходимости входит в FastComments и видит страницу согласия, где указано ваше приложение, учётная запись, к которой будет выполнено подключение, и запрошенные области доступа. Пользователь должен иметь разрешение **API Admin** в этой учётной записи; остальные увидят ошибку разрешения вместо формы согласия. При одобрении браузер перенаправляется на ваш `redirect_uri` с параметрами `code` и `state`. При отклонении происходит перенаправление с `error=access_denied`.

Код авторизации действителен 10 минут и может быть обменян один раз. Второй обмен тем же кодом отзывает все токены, полученные при первом обмене.

### Шаг 2 — Запрос токена

Обменяйте код на токены. Тело запроса закодировано как форма. Конфиденциальные клиенты аутентифицируются с помощью `client_secret_basic` (HTTP Basic) или `client_secret_post` (секрет в теле). Публичные клиенты отправляют только `client_id`.

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
    /** Только конфиденциальные клиенты. Может быть отправлен как HTTP Basic аутентификация. **/
    client_secret?: string
    code: string
    code_verifier: string
    /** Должен соответствовать запросу авторизации при отправке. **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа токена'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** С префиксом fcat_. Действителен один час. **/
    access_token: string
    token_type: 'bearer'
    /** Секунд до истечения срока действия токена доступа. 3600. **/
    expires_in: number
    /** С префиксом fcrt_. Действителен 30 дней с момента выдачи. **/
    refresh_token: string
    /** Разделённые пробелом предоставленные области доступа. **/
    scope: string
}
[inline-code-end]

Ошибки соответствуют RFC 6749: JSON‑тело с `error` и `error_description`, HTTP 400 для `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` и `unsupported_grant_type`, HTTP 401 для `invalid_client`, HTTP 429 при ограничении частоты запросов.

### Шаг 3 — Вызов API

Отправляйте токен доступа как токен Bearer. Тенант подразумевается токеном, поэтому `tenantId` является необязательным. Если указан, он должен соответствовать токену, иначе запрос завершится ошибкой.

[inline-code-attrs-start title = 'Пример cURL запроса с токеном Bearer'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` возвращает тенант, авторизующего пользователя и предоставленные области доступа, что делает его подходящим вызовом для теста соединения. Запрос с истёкшим или отозванным токеном получает HTTP 401. Запрос, метод которого требует области доступа, которой токен не обладает, получает HTTP 403.

### Шаг 4 — Обновление

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
    /** Необязательно. Ограничивает набор областей доступа, изначально предоставленных. **/
    scope?: string
    resource?: string
}
[inline-code-end]

Ответ имеет ту же структуру, что и обмен кода. Токены обновления вращаются: каждый запрос обновления возвращает новый `refresh_token` и отзывает старый после 30‑секундного окна ожидания для одновременных запросов. Предоставление токена обновления, который был вращён более чем 30 секунд назад, считается повтором и отзывает весь грант. Партнёрские приложения, зарегистрированные FastComments, освобождены от вращения и получают тот же токен обновления с продлённым сроком действия ещё на 30 дней.

Обновление также проверяет, что авторизующий пользователь всё ещё имеет разрешение API Admin в учётной записи. Если нет, грант отзывается, и ответ содержит `invalid_grant`.

### Отзыв

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

Отзыв refresh‑токена отзывает все токены доступа, выданные из того же гранта. Отзыв токена доступа отзывает только этот токен. Эндпоинт возвращает HTTP 200 с пустым JSON‑объектом независимо от того, найден токен или нет, согласно RFC 7009.

Пользователи также могут отозвать соединение в разделе **Connected Apps** в панели FastComments. Каждый токен для этого приложения сразу перестаёт работать.