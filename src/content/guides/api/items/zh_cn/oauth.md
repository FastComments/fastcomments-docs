FastComments 是一个 OAuth 2.1 授权服务器。应用程序可以获取一个绑定到某个 FastComments 账户的令牌，并在本指南中的每个端点上使用它来代替 API 密钥。Zapier 应用、MCP 服务器以及其他第三方集成就是通过这种方式连接的。

令牌通过带 PKCE 的授权码流程颁发。不支持客户端凭据授权或隐式授权。

### 发现

端点位置、支持的授权类型和认证方法发布在标准的元数据 URL 上：

[inline-code-attrs-start title = '授权服务器元数据'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

它所描述的端点：

[inline-code-attrs-start title = 'OAuth 端点'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

EU 区域的账户使用 `https://eu.fastcomments.com` 作为 issuer，路径相同。

### 注册客户端

客户端在开始流程之前需要一个 `client_id` 和一个已注册的 `redirect_uri`。有两种获取方式：

- **动态客户端注册。** 按照 RFC 7591 使用 JSON 请求体调用 `POST /oauth/register`（包含 `redirect_uris`、`client_name`、`client_uri`、`logo_uri`、`token_endpoint_auth_method`）。响应中包含 `client_id`，对于机密客户端还包括 `client_secret`。注册无需认证，并按 IP 进行速率限制。
- **客户端 ID 元数据文档。** 客户端使用其控制的 `https` URL 作为 `client_id`。FastComments 会抓取该 URL 并从中读取相同的元数据字段。无需调用注册接口。

FastComments 控制面板中列出的合作伙伴应用（如 Zapier）由 FastComments 直接注册。如果您正在构建市场应用列表并需要第一方客户端，请联系支持团队。

### 作用域

[inline-code-attrs-start title = '作用域'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET 和 HEAD 请求
write  其他所有方法
[inline-code-end]

未请求任何作用域的请求将同时获得两者。用户会在同意页面上看到所请求的作用域。请求这两个作用域之外的任何作用域都会失败并返回 `invalid_scope`。

### 第 1 步 - 授权请求

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
    /** 必须与客户端已注册的重定向 URI 之一完全匹配。 **/
    redirect_uri: string
    /** 以空格分隔。省略则请求两个作用域。 **/
    scope?: 'read' | 'write' | 'read write'
    /** 在重定向时原样返回。用它将回调绑定到发起流程的会话。 **/
    state?: string
    /** base64url(sha256(code_verifier))。 **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** 可选的 RFC 8707 资源指示器。如果发送，则必须向令牌端点发送相同的值。 **/
    resource?: string
}
[inline-code-end]

用户如需登录 FastComments，登录后会看到一个同意页面，其中列明您的应用程序、将要连接的账户以及所请求的作用域。用户必须在该账户上拥有 **API Admin** 权限；其他任何人都只会看到权限错误，而不是同意表单。批准后，浏览器将被重定向到您的 `redirect_uri`，并附带 `code` 和 `state`。拒绝则以 `error=access_denied` 重定向。

授权码有效期为 10 分钟，且只能兑换一次。对同一授权码的第二次兑换会撤销第一次兑换产生的所有令牌。

### 第 2 步 - 令牌请求

用授权码兑换令牌。请求体为表单编码。机密客户端使用 `client_secret_basic`（HTTP Basic）或 `client_secret_post`（密钥放在请求体中）进行认证。公共客户端只发送 `client_id`。

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

[inline-code-attrs-start title = '令牌请求体（authorization_code）'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestAuthorizationCode {
    grant_type: 'authorization_code'
    client_id: string
    /** 仅限机密客户端。也可以改用 HTTP Basic 认证发送。 **/
    client_secret?: string
    code: string
    code_verifier: string
    /** 如果发送，必须与授权请求中的值匹配。 **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = '令牌响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** 前缀为 fcat_。有效期为一小时。 **/
    access_token: string
    token_type: 'bearer'
    /** 访问令牌到期前的秒数。3600。 **/
    expires_in: number
    /** 前缀为 fcrt_。自颁发起有效期为 30 天。 **/
    refresh_token: string
    /** 以空格分隔的已授予作用域。 **/
    scope: string
}
[inline-code-end]

错误遵循 RFC 6749：返回包含 `error` 和 `error_description` 的 JSON 请求体；`invalid_request`、`invalid_grant`、`invalid_scope`、`invalid_target` 和 `unsupported_grant_type` 返回 HTTP 400；`invalid_client` 返回 HTTP 401；被速率限制时返回 HTTP 429。

### 第 3 步 - 调用 API

将访问令牌作为 bearer 令牌发送。租户由令牌隐含，因此 `tenantId` 是可选的。如果提供了该参数，它必须与令牌匹配，否则请求将失败。

[inline-code-attrs-start title = 'Bearer 令牌 cURL 示例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` 返回租户、授权用户以及已授予的作用域，因此它是连接测试的理想调用。使用已过期或已撤销令牌的请求会返回 HTTP 401。请求方法所需的作用域未被令牌持有时会返回 HTTP 403。

### 第 4 步 - 刷新

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

[inline-code-attrs-start title = '令牌请求体（refresh_token）'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestRefreshToken {
    grant_type: 'refresh_token'
    client_id: string
    client_secret?: string
    refresh_token: string
    /** 可选。将作用域缩小为最初授予的作用域的子集。 **/
    scope?: string
    resource?: string
}
[inline-code-end]

响应的结构与授权码兑换相同。刷新令牌会轮换：每次刷新都会返回一个新的 `refresh_token`，并在 30 秒的宽限期（用于并发请求）之后撤销旧令牌。提交一个已轮换超过 30 秒的刷新令牌会被视为重放攻击，并撤销整个授权。由 FastComments 注册的合作伙伴应用不受轮换限制，会返回相同的刷新令牌，其有效期再延长 30 天。

刷新时还会重新检查授权用户是否仍拥有该账户的 API Admin 权限。如果没有，授权将被撤销，响应为 `invalid_grant`。

### 撤销

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

撤销刷新令牌会撤销同一授权颁发的所有访问令牌。撤销访问令牌只撤销该令牌本身。无论是否找到该令牌，该端点都返回 HTTP 200 和一个空的 JSON 对象，遵循 RFC 7009。

用户还可以在 FastComments 控制面板的 **Connected Apps** 中撤销连接。该应用程序的所有令牌将立即失效。