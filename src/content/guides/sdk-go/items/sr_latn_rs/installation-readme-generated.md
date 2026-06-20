```bash
go get github.com/fastcomments/fastcomments-go
```

### Korišćenje API klijenta

#### Public API (Bez autentifikacije)

PublicAPI omogućava neautentifikovan pristup javnim endpointima:

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

    // Preuzmi komentare koristeći PublicAPI
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

#### Default API (Zahteva API ključ)

DefaultAPI zahteva autentifikaciju pomoću vašeg API ključa:

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

    // Kreiraj autentifikovan kontekst pomoću API ključa
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // Preuzmi komentare koristeći autentifikovani DefaultAPI
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

#### Moderation API (Kontrolna tabla moderatora)

ModerationAPI pokreće kontrolnu tablu moderatora. Pruža metode za listanje, brojanje, pretragu i izvoz komentara, akcije moderacije (uklanjanje/obnavljanje, označavanje, postavljanje statusa za pregled/spam/odobrenje, glasovi, ponovo otvaranje/zatvaranje niti), zabrane (zabrana komentarisanja, poništavanje, pregledi pre zabrane, status zabrane i podešavanja, brojevi zabranjenih korisnika), i značke i poverenje (dodela/uklanjanje znački, ručne značke, dobijanje/postavljanje faktora poverenja, interni profil korisnika). Sve Moderation metode prihvataju `sso` parametar za SSO-autentifikovane moderatore:

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

    // Lista komentara za moderaciju koristeći ModerationAPI
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