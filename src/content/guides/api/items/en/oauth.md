FastComments is an OAuth 2.1 authorization server. An application can obtain a token that is bound to one FastComments
account and use it on every endpoint in this guide in place of an API key. This is how the Zapier app, the MCP server,
and other third-party integrations connect.

Tokens are issued through the authorization code flow with PKCE. There is no client credentials or implicit grant.

### Discovery

Endpoint locations, supported grants, and auth methods are published at the standard metadata URL:

[inline-code-attrs-start title = 'Authorization Server Metadata'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

The endpoints it describes:

[inline-code-attrs-start title = 'OAuth Endpoints'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

Accounts on the EU region use `https://eu.fastcomments.com` as the issuer, with the same paths.

### Registering a client

A client needs a `client_id` and a registered `redirect_uri` before it can start the flow. There are two ways to get one:

- **Dynamic Client Registration.** `POST /oauth/register` with a JSON body per RFC 7591 (`redirect_uris`, `client_name`, `client_uri`, `logo_uri`, `token_endpoint_auth_method`). The response carries the `client_id` and, for confidential clients, the `client_secret`. Registration is unauthenticated and rate limited per IP.
- **Client ID Metadata Document.** The client uses an `https` URL it controls as its `client_id`. FastComments fetches that URL and reads the same metadata fields from it. No registration call is needed.

Partner applications listed in the FastComments dashboard, such as Zapier, are registered by FastComments directly. Contact support if you are building a marketplace listing and need a first-party client.

### Scopes

[inline-code-attrs-start title = 'Scopes'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

A request that asks for no scope is granted both. The user sees the requested scopes on the consent page. A request for a scope other than these two fails with `invalid_scope`.

### Step 1 - Authorization request

Send the user's browser to the authorize endpoint. PKCE with the `S256` method is required for every client.

[inline-code-attrs-start title = 'Authorization Request'; type = 'text'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Authorization Request Parameters'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

The user signs in to FastComments if needed and sees a consent page naming your application, the account it will be connected to, and the scopes requested. The user must hold the **API Admin** permission on that account; anyone else sees a permission error instead of the consent form. Approving redirects the browser to your `redirect_uri` with `code` and `state`. Denying redirects with `error=access_denied`.

The authorization code is valid for 10 minutes and can be exchanged once. A second exchange of the same code revokes every token the first exchange produced.

### Step 2 - Token request

Exchange the code for tokens. The body is form encoded. Confidential clients authenticate with `client_secret_basic` (HTTP Basic) or `client_secret_post` (secret in the body). Public clients send only `client_id`.

[inline-code-attrs-start title = 'Token Request cURL Example'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Token Request Body (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Token Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Errors follow RFC 6749: a JSON body with `error` and `error_description`, HTTP 400 for `invalid_request`, `invalid_grant`, `invalid_scope`, `invalid_target` and `unsupported_grant_type`, HTTP 401 for `invalid_client`, HTTP 429 when rate limited.

### Step 3 - Calling the API

Send the access token as a bearer token. The tenant is implied by the token, so `tenantId` is optional. When given it must match the token or the request fails.

[inline-code-attrs-start title = 'Bearer Token cURL Example'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` returns the tenant, the authorizing user, and the granted scopes, which makes it the right call for a connection test. A request with an expired or revoked token gets HTTP 401. A request whose method needs a scope the token does not hold gets HTTP 403.

### Step 4 - Refreshing

[inline-code-attrs-start title = 'Refresh Request cURL Example'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'Token Request Body (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

The response has the same shape as the code exchange. Refresh tokens rotate: each refresh returns a new `refresh_token` and revokes the old one after a 30 second grace window for concurrent requests. Presenting a refresh token that was rotated more than 30 seconds ago is treated as replay and revokes the whole grant. Partner applications registered by FastComments are exempt from rotation and get back the same refresh token with its expiry extended by another 30 days.

A refresh also re-checks that the authorizing user still holds API Admin on the account. If not, the grant is revoked and the response is `invalid_grant`.

### Revocation

[inline-code-attrs-start title = 'Revoke Request cURL Example'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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
