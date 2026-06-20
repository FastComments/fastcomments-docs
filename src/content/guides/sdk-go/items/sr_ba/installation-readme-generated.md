```bash
go get github.com/fastcomments/fastcomments-go
```

### Korištenje API klijenta

#### Javni API (Bez autentifikacije)

PublicAPI omogućava neautentifikovan pristup javnim endpoint-ima:

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

#### Default API (Zahtijeva API ključ)

DefaultAPI zahtijeva autentifikaciju koristeći vaš API ključ:

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

    // Kreiraj autentifikovani kontekst sa API ključem
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

ModerationAPI pokreće moderatorsku kontrolnu tablu. Pruža metode za listanje,
brojanje, pretraživanje, i izvoz komentara, moderacijske akcije (ukloni/vrati,
označi, postavi status pregled/spam/odobrenje, glasovi, ponovno otvori/zatvori niti), zabrane (zabrana komentarisanja,
poništi, sažeci prije zabrane, status i postavke zabrane, brojevi zabranjenih korisnika),
i značke & povjerenje (dodijeli/ukloni značke, ručne značke, dobavi/postavi faktor povjerenja, unutrašnji korisnički profil). Sve Moderation metode prihvataju `sso` parametar za
SSO-autentifikovane moderatore:

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

    // Dohvati komentare za moderaciju koristeći ModerationAPI
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