### 使用已驗證的 API (DefaultAPI)

**重要：** 已驗證的端點需要將您的 API 金鑰設定為 `x-api-key` 標頭。

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

### 使用審核 API (ModerationAPI)

審核端點為管理員儀表板提供功能，並使用代表執行審核操作的管理員的 SSO 令牌進行驗證：

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# 列出審核儀表板中的評論
let (response, httpResponse) = getApiComments(
  httpClient = client,
  page = 0,
  count = 30,
  textSearch = "",
  byIPFromComment = "",
  filters = "",
  searchFilters = "",
  sorts = "",
  demo = false,
  sso = "your-sso-token"
)

if response.isSome:
  let resp = response.get()
  echo "Found ", resp.comments.len, " comments"
```

### 常見問題

1. **401 authentication error**：請在進行 DefaultAPI 請求之前，確認已在您的 HttpClient 上設定 `x-api-key` 標頭：`client.headers["x-api-key"] = "your-api-key"`
2. **Wrong API class**：對於伺服器端需驗證的請求請使用 `api_default`，對於客戶端/公開請求請使用 `api_public`，而管理員儀表板的請求請使用 `api_moderation`。