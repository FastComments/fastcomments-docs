```bash
go get github.com/fastcomments/fastcomments-go
```

### API İstemcisinin Kullanımı

#### Public API (Kimlik Doğrulama Yok)

PublicAPI, kimlik doğrulaması gerektirmeyen genel uç noktalara erişim sağlar:

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

#### Default API (API Anahtarı Gerektirir)

DefaultAPI, API anahtarınızı kullanarak kimlik doğrulaması gerektirir:

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

    // API anahtarı ile kimlik doğrulamalı bağlam oluştur
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // Kimlik doğrulamalı DefaultAPI kullanarak yorumları al
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

#### Moderation API (Moderatör Paneli)

ModerationAPI, moderatör panelini çalıştırır. Yorumları listeleme, sayma, arama ve dışa aktarma; moderasyon işlemleri (kaldır/geri yükle, işaretle, inceleme/spam/onay durumunu ayarla, oylar, konuları tekrar aç/kapat); yasaklar (yorumu yasaklama, geri al, ön-yasak özetleri, yasak durumu ve tercihleri, yasaklı kullanıcı sayıları); ve rozetler & güven (rozet ver/kaldır, manuel rozetler, güven faktörünü al/ayar, kullanıcı iç profili) için yöntemler sağlar. Tüm Moderation yöntemleri SSO ile doğrulanmış moderatörler için `sso` parametresini kabul eder:

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

    // ModerationAPI kullanarak moderasyon için yorumları listele
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