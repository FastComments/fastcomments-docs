```bash
go get github.com/fastcomments/fastcomments-go
```

### Използване на API клиента

#### Public API (без удостоверяване)

PublicAPI позволява неавторизиран достъп до публични крайни точки:

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

    // Получаване на коментари чрез PublicAPI
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

#### Default API (Изисква API ключ)

DefaultAPI изисква удостоверяване чрез вашия API ключ:

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

    // Създаване на удостоверен контекст с API ключ
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // Получаване на коментари чрез удостоверения DefaultAPI
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

#### Moderation API (Табло на модератора)

ModerationAPI захранва таблото на модератора. Той предоставя методи за изброяване,
броене, търсене и експортиране на коментари, модераторски действия (премахване/възстановяване,
маркиране, задаване на статус преглед/спам/одобрение, гласове, повторно отваряне/затваряне на нишки), забрани (забрана за
коментиране, отмяна, предварителни резюмета преди забрана, статус и предпочитания за забрана, брой на забранени потребители),
и значки и доверие (награждаване/премахване на значки, ръчни значки, получаване/задаване на фактор на доверие, вътрешен
профил на потребителя). Всички модераторски методи приемат параметър `sso` за
модератори, удостоверени чрез SSO:

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

    // Извличане на коментари за модериране чрез ModerationAPI
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