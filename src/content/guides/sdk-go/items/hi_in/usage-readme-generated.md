### सरल SSO

```go
package main

import (
    "context"
    "fmt"
    "github.com/fastcomments/fastcomments-go/client"
    "github.com/fastcomments/fastcomments-go/sso"
)

func main() {
    // सरल SSO टोकन बनाएँ
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

    // प्रमाणित API कॉल करने के लिए SSO टोकन का उपयोग करें
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

### सुरक्षित SSO

```go
package main

import (
    "context"
    "fmt"
    "github.com/fastcomments/fastcomments-go/client"
    "github.com/fastcomments/fastcomments-go/sso"
)

func main() {
    // सुरक्षित SSO टोकन बनाएँ
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

    // प्रमाणित API कॉल करने के लिए SSO टोकन का उपयोग करें
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