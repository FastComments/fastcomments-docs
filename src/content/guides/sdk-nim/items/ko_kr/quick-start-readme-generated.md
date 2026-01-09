### 인증된 API 사용 (DefaultAPI)

**중요:** 인증된 엔드포인트는 API 키를 `x-api-key` 헤더로 설정해야 합니다.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# 인증된 API 호출
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

### 공개 API 사용 (PublicAPI)

공개 엔드포인트는 인증이 필요 없습니다:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# 공개 API 호출
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

### 일반적인 문제

1. **401 authentication error**: DefaultAPI 요청을 하기 전에 HttpClient에 `x-api-key` 헤더를 설정했는지 확인하세요: `client.headers["x-api-key"] = "your-api-key"`
2. **Wrong API class**: 서버 측 인증 요청에는 `api_default`를, 클라이언트 측/공개 요청에는 `api_public`을 사용하세요.