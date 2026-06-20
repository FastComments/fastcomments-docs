```bash
go get github.com/fastcomments/fastcomments-go
```

### Utilizzo del client API

#### Public API (Nessuna autenticazione)

La PublicAPI consente l'accesso non autenticato agli endpoint pubblici:

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

    // Recupera commenti usando PublicAPI
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

#### Default API (Richiede la chiave API)

La DefaultAPI richiede l'autenticazione utilizzando la tua chiave API:

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

    // Crea il contesto autenticato con la chiave API
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // Ottieni commenti usando la DefaultAPI autenticata
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

#### Moderation API (Dashboard del moderatore)

La ModerationAPI alimenta il pannello del moderatore. Fornisce metodi per elencare,
contare, cercare ed esportare commenti, azioni di moderazione (rimuovi/ripristina,
segnala, imposta stato revisione/spam/approvazione, voti, riapri/chiudi thread), ban (vietare dal
commentare, annulla, riepiloghi pre-ban, stato e preferenze del ban, conteggi utenti bannati),
e badge & fiducia (assegna/rimuovi badge, badge manuali, ottenere/impostare il fattore di fiducia, profilo interno utente). Tutti i metodi di Moderation accettano un parametro `sso` per
moderatori autenticati tramite SSO:

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

    // Elenca i commenti per moderazione usando la ModerationAPI
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