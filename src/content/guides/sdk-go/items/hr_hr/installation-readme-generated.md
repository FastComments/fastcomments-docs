```bash
go get github.com/fastcomments/fastcomments-go
```

### Korištenje API klijenta

#### Javni API (bez autentikacije)

The PublicAPI allows unauthenticated access to public endpoints:

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

    // Dohvati komentare koristeći PublicAPI
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

#### Default API (zahtijeva API ključ)

The DefaultAPI requires authentication using your API key:

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

    // Stvori autentificirani kontekst pomoću API ključa
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // Dohvati komentare koristeći autentificirani DefaultAPI
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

#### Moderation API (Nadzorna ploča moderatora)

The ModerationAPI powers the moderator dashboard. It provides methods for listing,
counting, searching, and exporting comments, moderation actions (remove/restore,
flag, set review/spam/approval status, votes, reopen/close threads), bans (ban from
comment, undo, pre-ban summaries, ban status and preferences, banned-user counts),
and badges & trust (award/remove badges, manual badges, get/set trust factor, user
internal profile). All Moderation methods accept an `sso` parameter for
SSO-authenticated moderators:

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

    // Nabavi popis komentara za moderaciju koristeći ModerationAPI
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