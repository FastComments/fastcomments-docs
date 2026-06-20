```bash
go get github.com/fastcomments/fastcomments-go
```

### 使用 API 用戶端

#### 公開 API（無需驗證）

PublicAPI 允許未經驗證地存取公開端點：

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

    // 使用 PublicAPI 取得評論
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

#### 預設 API（需要 API 金鑰）

DefaultAPI 需要使用您的 API 金鑰進行驗證：

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

    // 使用 API 金鑰建立已驗證的上下文
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // 使用已驗證的 DefaultAPI 取得評論
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

#### 審核 API（管理員儀表板）

ModerationAPI 提供管理員儀表板的功能。它提供用於列出、計數、搜尋和匯出評論的方法；審核操作（移除/還原、標記、設定審查/垃圾/核准狀態、投票、重新開啟/關閉討論串）；封鎖（禁止發言、復原、封鎖前摘要、封鎖狀態與偏好、被封鎖用戶計數）；以及徽章與信任（授予/移除徽章、手動徽章、取得/設定信任係數、使用者內部檔案）。所有 Moderation 方法都接受一個用於 SSO 驗證管理員的 `sso` 參數：

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

    // 使用 ModerationAPI 列出供審核的評論
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