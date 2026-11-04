FastComments 是一個 OAuth 2.1 授權伺服器。應用程式可以取得一個綁定到單一 FastComments 帳戶的權杖，並在本指南中的每個端點上使用它來取代 API 金鑰。Zapier 應用程式、MCP 伺服器以及其他第三方整合都是透過這種方式連線的。

權杖透過帶有 PKCE 的授權碼流程（authorization code flow）發放。不支援 client credentials 或隱含授權（implicit grant）。

### 探索

端點位置、支援的授權類型與驗證方法發布在標準的中繼資料 URL：

[inline-code-attrs-start title = '授權伺服器中繼資料'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

它所描述的端點：

[inline-code-attrs-start title = 'OAuth 端點'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET  https://fastcomments.com/oauth/authorize
POST https://fastcomments.com/oauth/token
POST https://fastcomments.com/oauth/revoke
POST https://fastcomments.com/oauth/register
[inline-code-end]

EU 區域的帳戶使用 `https://eu.fastcomments.com` 作為簽發者，路徑相同。

### 註冊用戶端

用戶端需要一個 `client_id` 和一個已註冊的 `redirect_uri` 才能開始流程。取得方式有兩種：

- **動態用戶端註冊（Dynamic Client Registration）。** 依照 RFC 7591，以 JSON 內容呼叫 `POST /oauth/register`（包含 `redirect_uris`、`client_name`、`client_uri`、`logo_uri`、`token_endpoint_auth_method`）。回應會包含 `client_id`，若是機密用戶端（confidential client）還會包含 `client_secret`。註冊無需驗證，並依 IP 進行速率限制。
- **用戶端 ID 中繼資料文件（Client ID Metadata Document）。** 用戶端使用其掌控的 `https` URL 作為 `client_id`。FastComments 會抓取該 URL 並從中讀取相同的中繼資料欄位。無需呼叫註冊端點。

FastComments 儀表板中列出的合作夥伴應用程式（例如 Zapier）由 FastComments 直接註冊。如果您正在建立市集上架項目並需要第一方用戶端，請聯絡支援團隊。

### 權限範圍（Scopes）

[inline-code-attrs-start title = '權限範圍'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
read   GET and HEAD requests
write  every other method
[inline-code-end]

未指定範圍的請求會同時取得兩者。使用者會在同意頁面上看到所請求的範圍。請求這兩個範圍以外的範圍將會失敗並回傳 `invalid_scope`。

### 步驟 1 - 授權請求

將使用者的瀏覽器導向授權端點。每個用戶端都必須使用 `S256` 方法的 PKCE。

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

使用者如有需要會先登入 FastComments，然後會看到一個同意頁面，其中列出您的應用程式、將連線的帳戶以及所請求的範圍。使用者必須在該帳戶上持有 **API Admin** 權限；其他人只會看到權限錯誤，而不是同意表單。核准後會將瀏覽器重新導向至您的 `redirect_uri`，並附帶 `code` 和 `state`。拒絕則會以 `error=access_denied` 重新導向。

授權碼有效期為 10 分鐘，且只能兌換一次。同一個授權碼第二次兌換將會撤銷第一次兌換所產生的所有權杖。

### 步驟 2 - 權杖請求

以權杖兌換授權碼。請求主體為表單編碼。機密用戶端使用 `client_secret_basic`（HTTP Basic）或 `client_secret_post`（金鑰放在主體中）進行驗證。公開用戶端僅傳送 `client_id`。

[inline-code-attrs-start title = '權杖請求 cURL 範例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = '權杖請求主體（authorization_code）'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = '權杖回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

錯誤遵循 RFC 6749：JSON 主體包含 `error` 和 `error_description`，`invalid_request`、`invalid_grant`、`invalid_scope`、`invalid_target` 與 `unsupported_grant_type` 回傳 HTTP 400，`invalid_client` 回傳 HTTP 401，被速率限制時回傳 HTTP 429。

### 步驟 3 - 呼叫 API

以 bearer 權杖方式傳送存取權杖。租戶由權杖隱含決定，因此 `tenantId` 為選填。若有提供，必須與權杖相符，否則請求會失敗。

[inline-code-attrs-start title = 'Bearer 權杖 cURL 範例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me' \
  --header 'Authorization: Bearer fcat_...'
[inline-code-end]

`GET /api/v1/me` 會回傳租戶、授權使用者以及授予的範圍，因此是連線測試的理想呼叫。使用已過期或已撤銷權杖的請求會得到 HTTP 401。請求方法所需的範圍若權杖未持有，則會得到 HTTP 403。

### 步驟 4 - 重新整理

[inline-code-attrs-start title = '重新整理請求 cURL 範例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/token' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'grant_type=refresh_token' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'refresh_token=fcrt_...'
[inline-code-end]

[inline-code-attrs-start title = '權杖請求主體（refresh_token）'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

回應的結構與授權碼兌換相同。重新整理權杖會輪替：每次重新整理都會回傳一個新的 `refresh_token`，並在 30 秒的寬限期（供並行請求使用）後撤銷舊的權杖。提交已輪替超過 30 秒的重新整理權杖會被視為重放攻擊（replay），並撤銷整個授權。由 FastComments 註冊的合作夥伴應用程式不受輪替限制，會取回相同的重新整理權杖，且其到期時間再延長 30 天。

重新整理時也會再次檢查授權使用者是否仍持有該帳戶的 API Admin 權限。若沒有，授權將被撤銷，回應為 `invalid_grant`。

### 撤銷

[inline-code-attrs-start title = '撤銷請求 cURL 範例'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/oauth/revoke' \
  --header 'Content-Type: application/x-www-form-urlencoded' \
  --data 'client_id=YOUR_CLIENT_ID' \
  --data 'client_secret=YOUR_CLIENT_SECRET' \
  --data 'token=fcrt_...' \
  --data 'token_type_hint=refresh_token'
[inline-code-end]

撤銷重新整理權杖會撤銷同一授權所發出的所有存取權杖。撤銷存取權杖僅撤銷該權杖本身。無論是否找到該權杖，此端點都會回傳 HTTP 200 與空的 JSON 物件，符合 RFC 7009。

使用者也可以從 FastComments 儀表板中的 **Connected Apps** 撤銷連線。該應用程式的所有權杖會立即失效。