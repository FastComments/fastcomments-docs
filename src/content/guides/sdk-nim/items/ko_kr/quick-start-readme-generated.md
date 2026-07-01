### Using Authenticated APIs (DefaultAPI)

**중요:** 인증된 엔드포인트는 API 키를 `x-api-key` 헤더에 설정해야 합니다.

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# 인증된 API 호출을 수행합니다.
# 필수 매개변수(및 요청 본문)는 위치 기반이며; 선택적
# 매개변수는 작업의 옵션 객체를 통해 전달됩니다.
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

### Using Public APIs (PublicAPI)

Public endpoints don't require authentication:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# 공개 API 호출을 수행합니다.
# tenantId와 urlId는 필수(위치 기반)이며; 나머지는 모두 선택 사항입니다.
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

### Using Moderation APIs (ModerationAPI)

Moderation endpoints power the moderator dashboard and are authenticated with an SSO token for the acting moderator:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# 조정 대시보드의 댓글을 나열합니다.
# 이 작업은 필수 매개변수가 없으므로 모든 것이 선택 사항입니다.
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

### Common Issues

1. **401 인증 오류**: DefaultAPI 요청을 하기 전에 HttpClient에 `x-api-key` 헤더를 설정했는지 확인하십시오: `client.headers["x-api-key"] = "your-api-key"`
2. **잘못된 API 클래스**: 서버 측 인증 요청에는 `api_default`를, 클라이언트 측/공개 요청에는 `api_public`을, 관리자 대시보드 요청에는 `api_moderation`을 사용하십시오.