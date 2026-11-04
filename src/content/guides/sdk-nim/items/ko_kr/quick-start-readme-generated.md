### 인증된 API 사용하기 (DefaultAPI)

**중요:** 인증된 엔드포인트를 사용하려면 API 키를 `x-api-key` 헤더로 설정해야 합니다.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# 인증된 API 호출을 수행합니다.
# 필수 파라미터(및 요청 본문)는 위치 인자로 전달하고, 선택적
# 파라미터는 해당 작업의 options 객체를 통해 전달합니다.
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

### 공개 API 사용하기 (PublicAPI)

공개 엔드포인트는 인증이 필요하지 않습니다:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# 공개 API를 호출합니다.
# tenantId와 urlId는 필수(위치 인자)이며, 그 외의 모든 것은 선택 사항입니다.
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

### 검토(Moderation) API 사용하기 (ModerationAPI)

검토 엔드포인트는 모더레이터 대시보드를 구동하며, 활동 중인 모더레이터의 SSO 토큰으로 인증됩니다:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# 검토 대시보드에서 댓글을 나열합니다.
# 이 작업에는 필수 파라미터가 없으므로 모든 항목이 선택 사항입니다.
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

### 일반적인 문제

1. **401 인증 오류**: DefaultAPI 요청을 보내기 전에 HttpClient에 `x-api-key` 헤더를 설정했는지 확인하세요: `client.headers["x-api-key"] = "your-api-key"`
2. **잘못된 API 클래스 사용**: 서버 측 인증 요청에는 `api_default`, 클라이언트 측/공개 요청에는 `api_public`, 모더레이터 대시보드 요청에는 `api_moderation`을 사용하세요.