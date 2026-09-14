FastComments 是一个 OAuth 2.1 授权服务器。应用程序可以获取绑定到单个 FastComments 账户的令牌，并在本指南的所有端点上使用该令牌代替 API 密钥。这就是 Zapier 应用、MCP 服务器以及其他第三方集成的连接方式。

令牌通过带 PKCE 的授权码流程颁发。不存在客户端凭证或隐式授权。

### Discovery

端点位置、支持的授权类型和认证方法在标准元数据 URL 上公布：

[inline-code-attrs-start title = '授权服务器元数据'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

它描述的端点如下：

[inline-code-attrs-start title = 'OAuth 端点'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

欧盟地区的账户使用 `https://eu.fastcomments.com` 作为发行者，路径相同。

### Registering a client

客户端在开始流程之前需要一个 `client_id` 和已注册的 `redirect_uri`。获取方式有两种：

- **动态客户端注册。** 使用符合 RFC 7591 的 JSON 请求体（`redirect_uris`、`client_name`、`client_uri`、`logo_uri`、`token_endpoint_auth_method`）通过 `POST /oauth/register`。响应中包含 `client_id`，对于机密客户端，还会返回 `client_secret`。注册无需认证，但每个 IP 有速率限制。
- **客户端 ID 元数据文档。** 客户端使用其控制的 `https` URL 作为 `client_id`。FastComments 会获取该 URL 并读取相同的元数据字段。无需进行注册调用。

在 FastComments 仪表板中列出的合作伙伴应用（如 Zapier）由 FastComments 直接注册。如果您正在构建市场列表并需要第一方客户端，请联系支持。

### Scopes

[inline-code-attrs-start title = '作用域'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

如果请求未指定作用域，则默认授予两者。用户在同意页面上会看到请求的作用域。请求除这两种之外的其他作用域会导致 `invalid_scope` 错误。

### Step 1 - Authorization request

将用户的浏览器重定向到授权端点。每个客户端都必须使用 `S256` 方法的 PKCE。

[inline-code-attrs-start title = '授权请求'; type = 'text'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = '授权请求参数'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

用户在需要时登录 FastComments，并看到一个同意页面，页面列出您的应用、将要连接的账户以及请求的作用域。用户必须在该账户上拥有 **API Admin** 权限；否则会看到权限错误而非同意表单。批准后浏览器会重定向到您的 `redirect_uri`，并附带 `code` 和 `state` 参数。拒绝则会重定向并返回 `error=access_denied`。

授权码有效期为 10 分钟，仅能兑换一次。第二次使用相同的授权码兑换会撤销第一次兑换产生的所有令牌。

### Step 2 - Token request

使用授权码兑换令牌。请求体采用表单编码。机密客户端使用 `client_secret_basic`（HTTP Basic）或 `client_secret_post`（在请求体中提供 secret）进行身份验证。公共客户端仅发送 `client_id`。

[inline-code-attrs-start title = '令牌请求 cURL 示例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = '令牌请求体 (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = '令牌响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

错误遵循 RFC 6749：返回包含 `error` 和 `error_description` 的 JSON 体，`invalid_request`、`invalid_grant`、`invalid_scope`、`invalid_target` 和 `unsupported_grant_type` 使用 HTTP 400，`invalid_client` 使用 HTTP 401，速率受限时返回 HTTP 429。

### Step 3 - Calling the API

将访问令牌作为 Bearer 令牌发送。租户信息由令牌隐含，因此 `tenantId` 为可选项。若提供，则必须与令牌匹配，否则请求失败。

[inline-code-attrs-start title = 'Bearer 令牌 cURL 示例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` 返回租户、授权用户以及已授予的作用域，这使其成为连接测试的合适调用。使用已过期或已撤销的令牌的请求会返回 HTTP 401。请求的方法需要令牌未持有的作用域时会返回 HTTP 403。

### Step 4 - Refreshing

[inline-code-attrs-start title = '刷新请求 cURL 示例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = '令牌请求体 (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

响应结构与代码兑换相同。刷新令牌会轮换：每次刷新返回一个新的 `refresh_token`，并在 30 秒的宽限期后撤销旧的令牌，以支持并发请求。提供已在 30 秒前轮换的刷新令牌会被视为重放，并撤销整个授权。由 FastComments 注册的合作伙伴应用免于轮换，并返回相同的刷新令牌，只是其有效期再延长 30 天。

刷新时还会重新检查授权用户是否仍在该账户上拥有 API Admin 权限。如果没有，授权将被撤销，响应为 `invalid_grant`。

### Revocation

[inline-code-attrs-start title = '撤销请求 cURL 示例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

撤销刷新令牌会撤销同一授权下所有已签发的访问令牌。撤销访问令牌仅撤销该令牌本身。该端点根据 RFC 7009，无论是否找到令牌，都返回 HTTP 200 并附带空的 JSON 对象。

用户也可以在 FastComments 仪表板的 **已连接应用** 中撤销连接。该应用的所有令牌会立即失效。

---