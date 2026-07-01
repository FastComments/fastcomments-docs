```bash
go get github.com/fastcomments/fastcomments-go
```

### API İstemcisini Kullanma

#### Genel API (Kimlik Doğrulama Yok)

PublicAPI, genel uç noktalara kimlik doğrulama gerektirmeden erişim sağlar:

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

    // PublicAPI kullanarak yorumları al
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

#### Varsayılan API (API Anahtarı Gerektirir)

DefaultAPI, API anahtarınızı kullanarak kimlik doğrulama gerektirir:

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

    // API anahtarı ile kimliği doğrulanmış bağlam oluştur
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // Kimliği doğrulanmış DefaultAPI kullanarak yorumları al
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

#### Moderasyon API (Moderatör Kontrol Paneli)

ModerasyonAPI, canlı ve hızlı moderasyon API'lerinin kapsamlı bir paketini sunar. Tüm moderasyon metodları bir `sso` parametresi kabul eder ve SSO veya bir FastComments.com oturum çerezi aracılığıyla kimlik doğrulayabilir:

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

    // ModerasyonAPI kullanarak moderasyon için yorumları listele
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