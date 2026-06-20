```bash
go get github.com/fastcomments/fastcomments-go
```

### Korišćenje API klijenta

#### Public API (bez autentifikacije)

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

#### Default API (Requires API Key)

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

    // Kreiraj autentifikovani kontekst pomoću API ključa
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // Dohvati komentare koristeći autentifikovani DefaultAPI
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

#### Moderation API (Moderatorska kontrolna tabla)

ModerationAPI pokreće moderatorsku kontrolnu tablu. On pruža metode za prikaz,
brojanje, pretraživanje i izvoz komentara, moderacione akcije (uklanjanje/obnavljanje,
oznakavanje, postavljanje statusa za pregled/spam/odobravanje, glasovi, ponovno otvaranje/zatvaranje tema), zabrane (zabrana od
komentarisanja, poništavanje, sažeci pre-zabrane, status zabrane i podešavanja, brojevi zabranjenih korisnika),
i značke & poverenje (dodeljivanje/uklanjanje znački, ručne značke, dobijanje/podešavanje faktora poverenja, korisnički
interni profil). Sve Moderation metode prihvataju parametar `sso` za
moderatore koji su autentifikovani putem SSO:

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

    // Prikaži komentare za moderaciju koristeći ModerationAPI
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