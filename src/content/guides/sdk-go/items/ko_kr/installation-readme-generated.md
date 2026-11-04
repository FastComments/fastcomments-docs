```bash
go get github.com/fastcomments/fastcomments-go
```

### API 클라이언트 사용하기

#### 퍼블릭 API (인증 불필요)

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

    // PublicAPI를 사용하여 댓글 가져오기
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

    // 인증된 DefaultAPI를 사용하여 댓글 가져오기
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

#### 모더레이션 API (모더레이터 대시보드)

ModerationAPI는 실시간 및 빠른 모더레이션 API를 폭넓게 제공합니다. 모든 모더레이션 메서드는 `sso` 파라미터를 받으며, SSO 또는 FastComments.com 세션 쿠키를 통해 인증할 수 있습니다:

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

    // ModerationAPI를 사용하여 모더레이션할 댓글 목록 가져오기
    response, httpResp, err := apiClient.ModerationAPI.GetApiComments(
        context.Background(),
    ).Sso("your-sso-token").Execute()

    if err != nil {
        panic(err)
    }

    fmt.Printf("Status: %d\n", httpResp.StatusCode)
    fmt.Printf("Comments: %+v\n", response)
}
```