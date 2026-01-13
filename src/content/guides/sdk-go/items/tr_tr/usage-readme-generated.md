### Basit SSO

```go
package main

import (
    "context"
    "fmt"
    "github.com/fastcomments/fastcomments-go/client"
    "github.com/fastcomments/fastcomments-go/sso"
)

func main() {
    // Basit SSO jetonu oluştur
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

    // Kimlik doğrulamalı bir API çağrısı yapmak için SSO jetonunu kullan
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

### Güvenli SSO

```go
package main

import (
    "context"
    "fmt"
    "github.com/fastcomments/fastcomments-go/client"
    "github.com/fastcomments/fastcomments-go/sso"
)

func main() {
    // Güvenli SSO jetonu oluştur
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

    // Kimlik doğrulamalı bir API çağrısı yapmak için SSO jetonunu kullan
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