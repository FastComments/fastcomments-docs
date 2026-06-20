```bash
go get github.com/fastcomments/fastcomments-go
```

### Verwendung des API-Clients

#### Public API (Keine Authentifizierung)

Die PublicAPI ermöglicht unauthentifizierten Zugriff auf öffentliche Endpunkte:

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

    // Kommentare mit der PublicAPI abrufen
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

#### Default API (Erfordert API-Schlüssel)

Die DefaultAPI erfordert die Authentifizierung mit Ihrem API-Schlüssel:

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

    // Authentifizierten Kontext mit API-Schlüssel erstellen
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // Kommentare mit der authentifizierten DefaultAPI abrufen
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

#### Moderation API (Moderator-Dashboard)

Die ModerationAPI treibt das Moderator-Dashboard an. Sie stellt Methoden zum Auflisten,
Zählen, Suchen und Exportieren von Kommentaren, Moderationsaktionen (entfernen/wiederherstellen,
melden, Überprüfungs-/Spam-/Genehmigungsstatus setzen, Bewertungen, Threads wieder öffnen/schließen), Sperren (vom
Kommentieren sperren, rückgängig machen, Pre-Bann‑Zusammenfassungen, Bannstatus und -präferenzen, Anzahl gesperrter Benutzer),
und Abzeichen & Vertrauen (Abzeichen vergeben/entfernen, manuelle Abzeichen, Vertrauensfaktor abrufen/setzen, internes Benutzerprofil) bereit. Alle Moderationsmethoden akzeptieren einen `sso`-Parameter für
SSO-authentifizierte Moderatoren:

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

    // Kommentare für die Moderation mit ModerationAPI auflisten
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