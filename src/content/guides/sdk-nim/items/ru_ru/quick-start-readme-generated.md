### Использование аутентифицированных API (DefaultAPI)

**Важно:** Аутентифицированные конечные точки требуют установки вашего API-ключа в заголовке `x-api-key`.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# Выполнить аутентифицированные вызовы API
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

### Использование публичных API (PublicAPI)

Публичные конечные точки не требуют аутентификации:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# Выполнить публичные вызовы API
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

### Использование модерационных API (ModerationAPI)

Модерационные конечные точки обеспечивают работу панели модератора и аутентифицируются с помощью SSO-токена действующего модератора:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# Получить список комментариев в панели модератора
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

### Распространённые проблемы

1. **401 authentication error**: Make sure you set the `x-api-key` header on your HttpClient before making DefaultAPI requests: `client.headers["x-api-key"] = "your-api-key"`
2. **Wrong API class**: Use `api_default` for server-side authenticated requests, `api_public` for client-side/public requests, and `api_moderation` for moderator dashboard requests.