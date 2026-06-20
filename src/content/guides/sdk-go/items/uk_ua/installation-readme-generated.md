```bash
go get github.com/fastcomments/fastcomments-go
```

### Використання клієнта API

#### Публічний API (без автентифікації)

PublicAPI дозволяє неавтентифікований доступ до публічних кінцевих точок:

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

    // Отримати коментарі за допомогою PublicAPI
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

#### Default API (потребує API-ключ)

DefaultAPI вимагає автентифікації за допомогою вашого API-ключа:

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

    // Створити автентифікований контекст з API-ключем
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // Отримати коментарі, використовуючи автентифікований DefaultAPI
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

ModerationAPI забезпечує роботу панелі модератора. Він надає методи для переліку,
підрахунку, пошуку та експорту коментарів, дій модерації (видалення/відновлення,
позначення, встановлення статусу для перегляду/спаму/підтвердження, голоси, повторне відкриття/закриття тем), банів (заборона коментування, скасування, підсумки перед баном, статус та налаштування бану, кількість забанених користувачів),
та значків і довіри (нагородження/видалення значків, ручні значки, отримання/встановлення коефіцієнта довіри, внутрішній профіль користувача). Усі методи Moderation приймають параметр `sso` для модераторів, автентифікованих через SSO:

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

    // Перелічити коментарі для модерації за допомогою ModerationAPI
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