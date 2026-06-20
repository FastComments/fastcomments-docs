```bash
go get github.com/fastcomments/fastcomments-go
```

### Brug af API-klienten

#### Public API (Ingen autentificering)

PublicAPI tillader adgang til offentlige endpoints uden autentificering:

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

    // Hent kommentarer ved hjælp af PublicAPI
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

#### Default API (Kræver API-nøgle)

DefaultAPI kræver autentificering ved hjælp af din API-nøgle:

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

    // Opret autentificeret kontekst med API-nøgle
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // Hent kommentarer ved hjælp af autentificeret DefaultAPI
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

#### Moderation API (Moderator-dashboard)

ModerationAPI driver moderator-dashboardet. Den leverer metoder til at liste, tælle, søge i og eksportere kommentarer, moderationshandlinger (fjern/gendan, marker, sæt review/spam/approval-status, stemmer, genåbn/luk tråde), bans (blokér fra at kommentere, fortryd, præ-ban-sammendrag, banestatus og præferencer, antal blokerede brugere), og badges & tillid (tildel/fjern badges, manuelle badges, få/sæt tillidsfaktor, brugerintern profil). Alle Moderation-metoder accepterer en `sso`-parameter for SSO-godkendte moderatorer:

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

    // Hent liste over kommentarer til moderation ved hjælp af ModerationAPI
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