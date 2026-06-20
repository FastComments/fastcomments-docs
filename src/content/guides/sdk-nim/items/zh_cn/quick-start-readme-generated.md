### 使用已认证的 API (DefaultAPI)

**重要：** 已认证的端点需要将您的 API 密钥设置为 `x-api-key` 头。

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# 发起已认证的 API 调用
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

### 使用公共 API (PublicAPI)

公共端点不需要认证：

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# 发起公共 API 调用
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

### 使用审核 API (ModerationAPI)

审核端点为版主仪表板提供支持，并使用代表版主的 SSO 令牌进行认证：

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# 在审核仪表板中列出评论
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

### 常见问题

1. **401 authentication error**：在发起 DefaultAPI 请求之前，确保在您的 HttpClient 上设置了 `x-api-key` 头：`client.headers["x-api-key"] = "your-api-key"`
2. **Wrong API class**：服务器端已认证请求使用 `api_default`，客户端/公共请求使用 `api_public`，版主仪表板请求使用 `api_moderation`。