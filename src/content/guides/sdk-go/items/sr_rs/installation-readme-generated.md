```bash
go get github.com/fastcomments/fastcomments-go
```

### Коришћење API клијента

#### Јавни API (Без аутентификације)

PublicAPI омогућава неаутентификован приступ јавним крајњим тачкама:

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

    // Преузмите коментаре помоћу PublicAPI
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

#### Подразумевани API (Захтева API кључ)

DefaultAPI захтева аутентификацију користећи ваш API кључ:

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

    // Креирајте аутентификован контекст са API кључем
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // Преузмите коментаре користећи аутентификовани DefaultAPI
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

#### API модерације (Контролна табла модератора)

ModerationAPI покреће контролну таблу модератора. Он пружа методе за листање,
бројање, претрагу и извоз коментара, радње модерације (уклањање/враћање,
означавање, подешавање статуса за преглед/спам/одобрење, гласови, поновно отварање/затварање тема), забране (забрана коментарисања,
опозив, прегледи пре забране, статус и преференције забране, бројеви забрањених корисника),
и значке и поверење (додавање/уклањање значки, ручне значке, добијање/подешавање фактора поверења, унутрашњи профил корисника). Све модераторске методе прихватају `sso` параметар за
SSO-аутентификоване модераторе:

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

    // Листајте коментаре за модерацију помоћу ModerationAPI
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