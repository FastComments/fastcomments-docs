All API methods in this SDK return tuples of `(Option[ResponseType], Response)`. 첫 번째 요소는 성공 시 파싱된 응답을 포함하고, 두 번째 요소는 원시 HTTP 응답입니다.

필수 매개변수와 요청 본문은 위치 기반으로 전달됩니다. 나머지 선택적 매개변수들은 단일 `Api<Operation>Options` 객체에 수집되며, 이는 마지막 인수입니다. 선택적 매개변수가 없는 작업은 옵션 객체를 사용하지 않습니다.

### 예제: 댓글 가져오기

```nim
import httpclient
import options
import fastcomments
import fastcomments/apis/api_default

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

let (response, httpResponse) = getComments(
  httpClient = client,
  tenantId = "your-tenant-id",
  options = GetCommentsOptions(
    urlId: "your-url-id",
    direction: SortDirections.DESC
  )
)

if httpResponse.code == Http200:
  if response.isSome:
    let resp = response.get()
    if resp.comments.isSome:
      echo "Found ", resp.comments.get().len, " comments"
```