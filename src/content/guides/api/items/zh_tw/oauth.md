FastComments 是一個 OAuth 2.1 授權伺服器。應用程式可以取得綁定至單一 FastComments
帳號的 token，並在本指南的所有端點上使用它來取代 API 金鑰。Zapier 應用、MCP 伺服器
以及其他第三方整合皆透過此方式連線。

Token 透過使用 PKCE 的授權碼流程發行。不存在 client credentials 或隱式授權。

### Discovery

端點位置、支援的授權類型與驗證方法皆在標準的 metadata URL 上公布：

[inline-code-attrs-start title = '授權伺服器元資料'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

它描述的端點：

[inline-code-attrs-start title = 'OAuth 端點'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

EU 區域的帳號使用 `https://eu.fastcomments.com` 作為發行者，路徑相同。

### Registering a client

客戶端在開始流程前需要一個 `client_id` 與已註冊的 `redirect_uri`。取得方式有兩種：

- **動態客戶端註冊。** 依 RFC 7591 使用 JSON 主體 `POST /oauth/register`（`redirect_uris`、`client_name`、`client_uri`、`logo_uri`、`token_endpoint_auth_method`）。回應會攜帶 `client_id`，對於機密客戶端，還會返回 `client_secret`。註冊不需驗證，且依 IP 受速率限制。
- **客戶端 ID 中繼文件。** 客戶端使用其控制的 `https` URL 作為 `client_id`。FastComments 會抓取該 URL，並從中讀取相同的中繼欄位。無需呼叫註冊。

在 FastComments 控制台列出的合作應用（如 Zapier）由 FastComments 直接註冊。若您正在建立市集列表並需要第一方客戶端，請聯絡支援。

### Scopes

[inline-code-attrs-start title = '範圍'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

未指定範圍的請求會同時授予兩者。使用者會在同意頁面看到請求的範圍。若請求的範圍不是上述兩者之一，則會回傳 `invalid_scope`。

### Step 1 - Authorization request

將使用者的瀏覽器導向授權端點。每個客戶端都必須使用 `S256` 方法的 PKCE。

[inline-code-attrs-start title = '授權請求'; type = 'text'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = '授權請求參數'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthAuthorizeQueryParams {
    response_type: 'code'
    client_id: string
    /** 必須完全符合客戶端已註冊的其中一個 redirect URI。 **/
    redirect_uri: string
    /** 以空格分隔。省略則同時請求兩個範圍。 **/
    scope?: 'read' | 'write' | 'read write'
    /** 在重新導向時原樣返回。用於將回呼綁定至啟動流程的會話。 **/
    state?: string
    /** base64url(sha256(code_verifier)). **/
    code_challenge: string
    code_challenge_method: 'S256'
    /** 可選的 RFC 8707 資源指示器。若傳送，必須在 token 端點也傳送相同值。 **/
    resource?: string
}
[inline-code-end]

使用者若需要會先登入 FastComments，並看到一個同意頁面，列出您的應用程式、將要連結的帳號，以及請求的範圍。使用者必須在該帳號上擁有 **API Admin** 權限；其他使用者會看到權限錯誤而非同意表單。批准後會將瀏覽器重新導向至您的 `redirect_uri`，附帶 `code` 與 `state`。拒絕則會以 `error=access_denied` 重新導向。

授權碼有效期為 10 分鐘，且只能兌換一次。第二次兌換相同的授權碼會撤銷第一次兌換所產生的所有 token。

### Step 2 - Token request

使用授權碼換取 token。請求主體為表單編碼。機密客戶端使用 `client_secret_basic`（HTTP Basic）或 `client_secret_post`（在主體中提供 secret）進行驗證。公開客戶端僅傳送 `client_id`。

[inline-code-attrs-start title = 'Token 請求 cURL 範例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Token 請求 Body (authorization_code)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestAuthorizationCode {
    grant_type: 'authorization_code'
    client_id: string
    /** 僅限機密客戶端。也可以改為使用 HTTP Basic 認證。 **/
    client_secret?: string
    code: string
    code_verifier: string
    /** 必須與授權請求中傳送的相符。 **/
    redirect_uri?: string
    resource?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Token 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenResponse {
    /** 前綴 fcat_。有效期一小時。 **/
    access_token: string
    token_type: 'bearer'
    /** 距離存取 token 到期的秒數。3600。 **/
    expires_in: number
    /** 前綴 fcrt_。自發行日起有效期 30 天。 **/
    refresh_token: string
    /** 以空格分隔的已授予範圍。 **/
    scope: string
}
[inline-code-end]

錯誤遵循 RFC 6749：回傳包含 `error` 與 `error_description` 的 JSON 主體，`invalid_request`、`invalid_grant`、`invalid_scope`、`invalid_target`、`unsupported_grant_type` 回傳 HTTP 400，`invalid_client` 回傳 HTTP 401，速率受限時回傳 HTTP 429。

### Step 3 - Calling the API

將存取 token 作為 bearer token 傳送。租戶資訊由 token 隱含，因此 `tenantId` 為可選。若提供，必須與 token 中的租戶相符，否則請求失敗。

[inline-code-attrs-start title = 'Bearer Token cURL 範例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` 會回傳租戶、授權使用者以及已授予的範圍，是測試連線的合適呼叫。使用過期或已撤銷的 token 會得到 HTTP 401。若請求的方法需要 token 未持有的範圍，則會得到 HTTP 403。

### Step 4 - Refreshing

[inline-code-attrs-start title = 'Refresh 請求 cURL 範例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = 'Token 請求 Body (refresh_token)'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface OAuthTokenRequestRefreshToken {
    grant_type: 'refresh_token'
    client_id: string
    client_secret?: string
    refresh_token: string
    /** 可選。縮小至最初授予的子集合範圍。 **/
    scope?: string
    resource?: string
}
[inline-code-end]

回應與授權碼交換的結構相同。Refresh token 會輪替：每次刷新都會返回新的 `refresh_token`，並在 30 秒寬限期後撤銷舊的 token，以支援同時請求。若提供的 refresh token 已在 30 秒前被輪替，則視為重放攻擊，並撤銷整個授權。由 FastComments 註冊的合作應用免於輪替，會返回相同的 refresh token，且其有效期再延長 30 天。

刷新同時也會重新檢查授權使用者是否仍在該帳號上擁有 API Admin 權限。若沒有，授權會被撤銷，回應為 `invalid_grant`。

### Revocation

[inline-code-attrs-start title = 'Revoke 請求 cURL 範例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

撤銷 refresh token 會撤銷同一授權所產生的所有 access token。撤銷 access token 只會撤銷該 token。本端點無論是否找到 token，都會回傳 HTTP 200 且空的 JSON 物件，符合 RFC 7009。

使用者也可以在 FastComments 控制台的 **已連結的應用** 中撤銷連線。該應用的所有 token 會立即失效。