### 使用已驗證的 API (DefaultAPI)

**重要：** 已驗證的端點需要您將 API 金鑰設為 `x-api-key` 標頭。

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# 執行已驗證的 API 呼叫
let (response, httpResponse) = getComments(
  httpClient = client,
  tenantId = "your-tenant-id",
  page = 0,
  limit = 0,
  skip = 0,
  asTree = false,
  skipChildren = 0,
  limitChildren = 0,
  maxTreeDepth = 0,
  urlId = "your-url-id",
  userId = "",
  anonUserId = "",
  contextUserId = "",
  hashTag = "",
  parentId = "",
  direction = SortDirections.DESC
)

if response.isSome:
  let resp = response.get()
  if resp.comments.isSome:
    echo "Found ", resp.comments.get().len, " comments"
```

### 使用公開 API (PublicAPI)

公開端點不需要驗證：

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# 執行公開 API 呼叫
let (response, httpResponse) = getCommentsPublic(
  httpClient = client,
  tenantId = "your-tenant-id",
  urlId = "your-url-id",
  page = 0,
  direction = SortDirections.DESC,
  sso = "",
  skip = 0,
  skipChildren = 0,
  limit = 0,
  limitChildren = 0,
  countChildren = false,
  fetchPageForCommentId = "",
  includeConfig = false,
  countAll = false,
  includei10n = false,
  locale = "",
  modules = "",
  isCrawler = false,
  includeNotificationCount = false,
  asTree = false,
  maxTreeDepth = 0,
  useFullTranslationIds = false,
  parentId = "",
  searchText = "",
  hashTags = @[],
  userId = "",
  customConfigStr = "",
  afterCommentId = "",
  beforeCommentId = ""
)

if response.isSome:
  let resp = response.get()
  if resp.comments.isSome:
    echo "Found ", resp.comments.get().len, " comments"
```

### 常見問題

1. **401 認證錯誤**：在發出 DefaultAPI 請求之前，請確認已在 HttpClient 上設定 `x-api-key` 標頭：`client.headers["x-api-key"] = "your-api-key"`
2. **錯誤的 API 類別**：伺服器端的已驗證請求請使用 `api_default`，用於客戶端/公開的請求請使用 `api_public`。
---