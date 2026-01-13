### 簡單 SSO

```go
package main

import (
    "context"
    "fmt"
    "github.com/fastcomments/fastcomments-go/client"
    "github.com/fastcomments/fastcomments-go/sso"
)

func main() {
    // 建立簡單 SSO 令牌
    user := &sso.SimpleSSOUserData{
        UserID: "user-123",
        Email:  "user@example.com",
        Avatar: "https://example.com/avatar.jpg",
    }

    ssoInstance := sso.NewSimple(user)
    token, err := ssoInstance.CreateToken()
    if err != nil {
        panic(err)
    }

    fmt.Println("SSO Token:", token)

    // 使用 SSO 令牌進行經過驗證的 API 呼叫
    config := client.NewConfiguration()
    apiClient := client.NewAPIClient(config)

    response, httpResp, err := apiClient.PublicAPI.GetCommentsPublic(
        context.Background(),
        "your-tenant-id",
    ).UrlId("your-page-url-id").Sso(token).Execute()

    if err != nil {
        panic(err)
    }

    fmt.Printf("Status: %d\n", httpResp.StatusCode)
    fmt.Printf("Comments: %+v\n", response)
}
```

### 安全 SSO

```go
package main

import (
    "context"
    "fmt"
    "github.com/fastcomments/fastcomments-go/client"
    "github.com/fastcomments/fastcomments-go/sso"
)

func main() {
    // 建立安全 SSO 令牌
    user := &sso.SecureSSOUserData{
        UserID:   "user-123",
        Email:    "user@example.com",
        Username: "johndoe",
        Avatar:   "https://example.com/avatar.jpg",
    }

    apiKey := "your-api-key"
    ssoInstance, err := sso.NewSecure(apiKey, user)
    if err != nil {
        panic(err)
    }

    token, err := ssoInstance.CreateToken()
    if err != nil {
        panic(err)
    }

    fmt.Println("Secure SSO Token:", token)

    // 使用 SSO 令牌進行經過驗證的 API 呼叫
    config := client.NewConfiguration()
    apiClient := client.NewAPIClient(config)

    response, httpResp, err := apiClient.PublicAPI.GetCommentsPublic(
        context.Background(),
        "your-tenant-id",
    ).UrlId("your-page-url-id").Sso(token).Execute()

    if err != nil {
        panic(err)
    }

    fmt.Printf("Status: %d\n", httpResp.StatusCode)
    fmt.Printf("Comments: %+v\n", response)
}
```