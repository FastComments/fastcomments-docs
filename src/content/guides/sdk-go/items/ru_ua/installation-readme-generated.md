```bash
go get github.com/fastcomments/fastcomments-go
```

### Использование API-клиента

#### Публичный API (без аутентификации)

PublicAPI предоставляет неаутентифицированный доступ к публичным конечным точкам:

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

    // Получить комментарии с помощью PublicAPI
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

#### Default API (требуется API-ключ)

DefaultAPI требует аутентификации с использованием вашего API-ключа:

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

    // Создать аутентифицированный контекст с API-ключом
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // Получить комментарии, используя аутентифицированный DefaultAPI
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

#### Moderation API (Панель модератора)

ModerationAPI обслуживает панель модератора. Он предоставляет методы для перечисления,
подсчёта, поиска и экспорта комментариев, модерационных действий (удаление/восстановление,
пометка, установка статуса на проверку/спам/одобрение, голосование, повторное открытие/закрытие веток),
блокировок (запрет на комментирование, отмена, предварительные сводки по блокировкам, статус и настройки блокировок, количество заблокированных пользователей),
и бейджей & доверия (назначение/удаление бейджей, ручные бейджи, получение/установка коэффициента доверия, внутренний профиль пользователя). Все методы Moderation принимают параметр `sso` для
модераторов, аутентифицированных через SSO:

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

    // Получить список комментариев для модерации с помощью ModerationAPI
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