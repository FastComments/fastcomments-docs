```bash
go get github.com/fastcomments/fastcomments-go
```

### Korzystanie z klienta API

#### Publiczne API (bez uwierzytelniania)

Publiczne API umożliwia nieautoryzowany dostęp do publicznych punktów końcowych:

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

    // Get comments using PublicAPI
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

#### Domyślne API (wymaga klucza API)

Domyślne API wymaga uwierzytelnienia przy użyciu klucza API:

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

    // Create authenticated context with API key
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // Get comments using authenticated DefaultAPI
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

#### API moderacji (Panel moderatora)

ModerationAPI zasila panel moderatora. Udostępnia metody do listowania,
zliczania, wyszukiwania i eksportowania komentarzy, działania moderacyjne (usuń/przywróć,
oznacz, ustaw status do recenzji/spam/zaakceptowany, głosy, ponowne otwarcie/zamknięcie wątków), bany (zablokuj komentowanie,
cofnij, podsumowania przed banem, status i preferencje banów, liczby zbanowanych użytkowników),
oraz odznaki i zaufanie (przyznawanie/usuwanie odznak, odznaki ręczne, pobierz/ustaw współczynnik zaufania, wewnętrzny profil użytkownika). Wszystkie metody Moderation akceptują parametr `sso` dla
moderatorów uwierzytelnionych przez SSO:

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

    // List comments for moderation using ModerationAPI
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