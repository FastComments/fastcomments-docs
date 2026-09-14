FastComments је OAuth 2.1 сервер за ауторизацију. Апликација може добити токен који је везан за један FastComments налог и користити га на сваком крајњем тачком у овом водичу уместо API кључа. Овако се Zapier апликација, MCP сервер и остале интеграције трећих страна повезују.

Токени се издају кроз токен кодни токен (authorization code) током са PKCE. Не постоје client credentials или implicit grant.

### Откривање

Endpoint locations, supported grants, and auth methods are published at the standard metadata URL:

[inline-code-attrs-start title = 'Метаподаци сервера за ауторизацију'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

The endpoints it describes:

[inline-code-attrs-start title = 'OAuth крајње тачке'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Accounts on the EU region use `https://eu.fastcomments.com` as the issuer, with the same paths.

### Регистрација клијента

A client needs a `client_id` and a registered `redirect_uri` before it can start the flow. There are two ways to get one:

- **Dynamic Client Registration.** `POST /oauth/register` with a JSON body per RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). The response carries the `client_id` and, for confidential clients, the `client_secret`. Registration is unauthenticated and rate limited per IP.
- **Client ID Metadata Document.** The client uses an `https` URL it controls as its `client_id`. FastComments fetches that URL and reads the same metadata fields from it. No registration call is needed.

Partner applications listed in the FastComments dashboard, such as Zapier, are registered by FastComments directly. Contact support if you are building a marketplace listing and need a first-party client.

### Обим

[inline-code-attrs-start title = 'Обим'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

A request that asks for no scope is granted both. The user sees the requested scopes on the consent page. A request for a scope other than these two fails with `invalid_scope`.

### Корак 1 – Захтев за ауторизацију

Send the user's browser to the authorize endpoint. PKCE with the `S256` method is required for every client.

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
    /** Мора тачно да се поклапа са једном од регистрованих redirect URI-ја клијента. **/
    redirect_uri: string
    /** Раздвојено размаком. Изоставите за захтев за оба обима. **/
    scope?: 'read' | 'write' | 'read write'
    /** Враћено неизмењено у редиректу. Користите га за везу повратног позива за сесију која је покренула ток. **/
    state?: string
    /** base64url(sha256(code_verifier)). **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** Опционални RFC 8707 индикатор ресурса. Ако се пошаље, иста вредност мора бити послата токен крајњој тачки. **/
    resource?: string
}
[inline-code-end]

The user signs in to FastComments if needed and sees a consent page naming your application, the account it will be connected to, and the scopes requested. The user must hold the **API Admin** permission on that account; anyone else sees a permission error instead of the consent form. Approving redirects the browser to your `redirect_uri` with `code` and `state`. Denying redirects with `error=access_denied`.

The authorization code is valid for 10 minutes and can be exchanged once. A second exchange of the same code revokes every token the first exchange produced.

### Корак 2 – Захтев за токен

Exchange the code for tokens. The body is form encoded. Confidential clients authenticate with `client_secret_basic` (HTTP Basic) or `client_secret_post` (secret in the body). Public clients send only `client_id`.

[inline-code-attrs-start title = 'Пример cURL захтева за токен'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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
    /** Само за поверљиве клијенте. Може се послати као HTTP Basic аутентикација уместо тога. **/
    client_secret?: string
    code: string
    code_verifier: string
    /** Мора да се поклапа са захтевом за ауторизацију када се пошаље. **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора токена'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** Префикс fcat_. Важи један сат. **/
    access_token: string
    token_type: 'bearer'
    /** Секунди до истека приступног токена. 3600. **/
    expires_in: number
    /** Префикс fcrt_. Важи 30 дана од издавања. **/
    refresh_token: string
    /** Раздвојени размаком одобрени обими. **/
    scope: string
}
[inline-code-end]

Errors follow RFC 6749: a JSON body with `error` and `error_description`, HTTP 400 for `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` and `unsupported_grant_type`, HTTP 401 for `invalid_client`, HTTP 429 when rate limited.

### Корак 3 – Позивање API‑ја

Send the access token as a bearer token. The tenant is implied by the token, so `tenantId` is optional. When given it must match the token or the request fails.

[inline-code-attrs-start title = 'Пример cURL захтева за носио токен'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` returns the tenant, the authorizing user, and the granted scopes, which makes it the right call for a connection test. A request with an expired or revoked token gets HTTP 401. A request whose method needs a scope the token does not hold gets HTTP 403.

### Корак 4 – Освежавање

[inline-code-attrs-start title = 'Пример cURL захтева за освежавање'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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
    /** Опционално. Сужава на подскуп оригинално одобрених обима. **/
    scope?: string
    resource?: string
}
[inline-code-end]

The response has the same shape as the code exchange. Refresh tokens rotate: each refresh returns a new `refresh_token` and revokes the old one after a 30 second grace window for concurrent requests. Presenting a refresh token that was rotated more than 30 seconds ago is treated as replay and revokes the whole grant. Partner applications registered by FastComments are exempt from rotation and get back the same refresh token with its expiry extended by another 30 days.

A refresh also re-checks that the authorizing user still holds API Admin on the account. If not, the grant is revoked and the response is `invalid_grant`.

### Опозив

[inline-code-attrs-start title = 'Пример cURL захтева за опозив'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

Revoking a refresh token revokes every access token issued from the same grant. Revoking an access token revokes only that token. The endpoint returns HTTP 200 with an empty JSON object whether or not the token was found, per RFC 7009.

Users can also revoke a connection from **Connected Apps** in the FastComments dashboard. Every token for that application stops working immediately.

---