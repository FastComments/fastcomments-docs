```bash
go get github.com/fastcomments/fastcomments-go
```

### De API Client gebruiken

#### Public API (Geen authenticatie)

De Public API biedt niet-geauthenticeerde toegang tot publieke endpoints:

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

    // Reacties ophalen via PublicAPI
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

#### Default API (Vereist API-sleutel)

De DefaultAPI vereist authenticatie met uw API-sleutel:

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

    // Maak een geauthenticeerde context met API-sleutel
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // Reacties ophalen via geauthenticeerde DefaultAPI
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

#### Moderation API (Moderator Dashboard)

De ModerationAPI voedt het moderator-dashboard. Het biedt methoden voor het weergeven, tellen, zoeken en exporteren van reacties, moderatieacties (verwijderen/terugzetten, markeren, review/spam/goedkeuringsstatus instellen, stemmen, draadjes heropenen/sluiten), bans (verbod om te reageren, ongedaan maken, pre-ban-samenvattingen, ban-status en voorkeuren, aantal verbannen gebruikers), en badges & vertrouwen (badges toekennen/verwijderen, handmatige badges, trustfactor ophalen/instellen, intern gebruikersprofiel). Alle Moderation-methoden accepteren een `sso`-parameter voor SSO-geauthenticeerde moderators:

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

    // Reacties ophalen voor moderatie met ModerationAPI
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