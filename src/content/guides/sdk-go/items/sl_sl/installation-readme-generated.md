```bash
go get github.com/fastcomments/fastcomments-go
```

### Uporaba API odjemalca

#### Javni API (brez overjanja)

PublicAPI omogoča neavtenticiran dostop do javnih končnih točk:

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

    // Pridobite komentarje z uporabo PublicAPI
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

#### Privzeti API (zahteva API ključ)

DefaultAPI zahteva avtentikacijo z vašim API ključem:

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

    // Ustvarite avtenticiran kontekst z API ključem
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // Pridobite komentarje z uporabo avtenticiranega DefaultAPI
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

#### Moderation API (nadzorna plošča moderatorja)

ModerationAPI poganja nadzorno ploščo moderatorja. Ponuja metode za seznam,
štetje, iskanje in izvoz komentarjev, moderacijske ukrepe (odstrani/obnovi,
označi, nastavi status pregleda/špama/odobritve, glasovanja, ponovno odpri/zaključi niti), prepovedi (prepoved komentiranja,
razveljavitev, povzetki pred prepovedjo, status in nastavitve prepovedi, število prepovedanih uporabnikov),
ter značke in zaupanje (podeljevanje/odstranjevanje značk, ročne značke, pridobitev/nastavitev faktorja zaupanja, notranji profil uporabnika). Vse Moderation metode sprejemajo parameter `sso` za
moderatorje, avtenticirane prek SSO:

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

    // Pridobite komentarje za moderacijo z uporabo ModerationAPI
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