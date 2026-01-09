```bash
go get github.com/fastcomments/fastcomments-go
```

### API 클라이언트 사용

#### 퍼블릭 API (인증 없음)

PublicAPI는 인증 없이 공개 엔드포인트에 접근할 수 있습니다:

```go
package main

import (
    "context"
    "fmt"
    "github.com/fastcomments/fastcomments-go/client"
)

func main() {
    config := client.NewConfiguration()
    apiClient := client.NewAPIClient(config)

    // PublicAPI를 사용해 댓글 가져오기
    response, httpResp, err := apiClient.PublicAPI.GetCommentsPublic(
        context.Background(),
        "your-tenant-id",
    ).UrlId("your-page-url-id").Execute()

    if err != nil {
        panic(err)
    }

    fmt.Printf("Status: %d\n", httpResp.StatusCode)
    fmt.Printf("Comments: %+v\n", response)
}
```

#### 기본 API (API 키 필요)

DefaultAPI는 API 키를 사용한 인증이 필요합니다:

```go
package main

import (
    "context"
    "fmt"
    "github.com/fastcomments/fastcomments-go/client"
)

func main() {
    config := client.NewConfiguration()
    apiClient := client.NewAPIClient(config)

    // API 키로 인증된 컨텍스트 생성
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // 인증된 DefaultAPI로 댓글 가져오기
    response, httpResp, err := apiClient.DefaultAPI.GetComments(auth).
        TenantId("your-tenant-id").
        UrlId("your-page-url-id").
        Execute()

    if err != nil {
        panic(err)
    }

    fmt.Printf("Status: %d\n", httpResp.StatusCode)
    fmt.Printf("Comments: %+v\n", response)
}
```