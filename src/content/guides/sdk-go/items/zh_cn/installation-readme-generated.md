```bash
go get github.com/fastcomments/fastcomments-go
```

### 使用 API 客户端

#### 公共 API（无需身份验证）

PublicAPI 允许未认证访问公共端点：

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

    // 使用 PublicAPI 获取评论
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

#### 默认 API（需要 API 密钥）

DefaultAPI 需要使用您的 API 密钥进行身份验证：

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

    // 使用 API 密钥创建已认证的上下文
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // 使用已认证的 DefaultAPI 获取评论
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

#### Moderation API（版主仪表板）

ModerationAPI 为版主仪表板提供支持。它提供用于列出、计数、搜索和导出评论的接口、审核操作（移除/恢复、标记、设置审核/垃圾/批准状态、投票、重新打开/关闭线程）、封禁（禁止评论、撤销、预封禁摘要、封禁状态和偏好、被封禁用户计数）以及徽章与信任（授予/移除徽章、手动徽章、获取/设置信任因子、用户内部资料）。所有 Moderation 方法都接受一个 `sso` 参数，用于 SSO 认证的版主：

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

    // 使用 ModerationAPI 列出待审核的评论
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