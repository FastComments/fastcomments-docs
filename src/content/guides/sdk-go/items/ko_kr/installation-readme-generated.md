```bash
go get github.com/fastcomments/fastcomments-go
```

### API 클라이언트 사용

#### 공개 API (인증 없음)

PublicAPI는 인증 없이 공용 엔드포인트에 접근할 수 있습니다:

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

    // PublicAPI로 댓글 가져오기
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

DefaultAPI는 API 키를 사용한 인증을 필요로 합니다:

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

#### 중재 API (모더레이터 대시보드)

ModerationAPI는 모더레이터 대시보드를 구동합니다. 댓글 목록 작성, 개수 세기, 검색 및 내보내기, 중재 작업(삭제/복원, 플래그, 검토/스팸/승인 상태 설정, 투표, 스레드 재오픈/닫기), 차단(댓글 차단, 취소, 사전 차단 요약, 차단 상태 및 환경 설정, 차단된 사용자 수) 및 배지 및 신뢰(배지 수여/삭제, 수동 배지, 신뢰도 가져오기/설정, 사용자 내부 프로필) 등을 위한 메서드를 제공합니다. 모든 Moderation 메서드는 SSO로 인증된 모더레이터를 위한 `sso` 매개변수를 허용합니다:

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

    // ModerationAPI로 중재용 댓글 목록 가져오기
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