### 使用已驗證的 API（DefaultAPI）

**重要**：已驗證的端點需要將您的 API 金鑰設定為 `x-api-key` 標頭。

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Make authenticated API calls.
# Required parameters (and the request body) are positional; optional
# parameters are passed via the operation's options object.
let (response, httpResponse) = getComments(
  httpClient = client,
  tenantId = "your-tenant-id",
  options = GetCommentsOptions(
    urlId: "your-url-id",
    direction: SortDirections.DESC
  )
)

if response.isSome:
  let resp = response.get()
  if resp.comments.isSome:
    echo "Found ", resp.comments.get().len, " comments"
```

### 使用公共 API（PublicAPI）

公共端點不需要驗證：

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Make public API calls.
# tenantId and urlId are required (positional); everything else is optional.
let (response, httpResponse) = getCommentsPublic(
  httpClient = client,
  tenantId = "your-tenant-id",
  urlId = "your-url-id",
  options = GetCommentsPublicOptions(
    direction: SortDirections.DESC
  )
)

if response.isSome:
  let resp = response.get()
  if resp.comments.isSome:
    echo "Found ", resp.comments.get().len, " comments"
```

### 使用審核 API（ModerationAPI）

審核端點為審核者儀表板提供功能，並以執行中的審核者的 SSO 令牌進行驗證：

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# List comments in the moderation dashboard.
# This operation has no required parameters, so everything is optional.
let (response, httpResponse) = getApiComments(
  httpClient = client,
  options = GetApiCommentsOptions(
    count: 30,
    tenantId: "your-tenant-id",
    sso: "your-sso-token"
  )
)

if response.isSome:
  let resp = response.get()
  echo "Found ", resp.comments.len, " comments"
```

### 常見問題

1. **401 認證錯誤**：確保在發送 DefaultAPI 請求之前，在 HttpClient 上設置 `x-api-key` 標頭：`client.headers["x-api-key"] = "your-api-key"`
2. **錯誤的 API 類別**：對於伺服器端已驗證的請求使用 `api_default`，對於客戶端/公共請求使用 `api_public`，以及對於審核者儀表板請求使用 `api_moderation`。