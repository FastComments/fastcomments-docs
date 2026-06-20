### 인증된 API 사용 (DefaultAPI)

**중요:** 인증된 엔드포인트는 `x-api-key` 헤더에 API 키가 설정되어 있어야 합니다.

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

공개 엔드포인트는 인증이 필요하지 않습니다:

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

### 모더레이션 API 사용 (ModerationAPI)

모더레이션 엔드포인트는 모더레이터 대시보드를 구동하며, 해당 모더레이터의 SSO 토큰으로 인증됩니다:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# 모더레이션 대시보드의 댓글 목록
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

### 일반적인 문제

1. **401 인증 오류**: DefaultAPI 요청을 하기 전에 HttpClient에 `x-api-key` 헤더를 설정했는지 확인하세요: `client.headers["x-api-key"] = "your-api-key"`
2. **잘못된 API 클래스**: 서버 측 인증된 요청에는 `api_default`를 사용하고, 클라이언트 측/공개 요청에는 `api_public`을 사용하며, 모더레이터 대시보드 요청에는 `api_moderation`을 사용하세요.