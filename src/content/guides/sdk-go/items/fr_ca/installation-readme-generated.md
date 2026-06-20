```bash
go get github.com/fastcomments/fastcomments-go
```

### Utilisation du client de l'API

#### API publique (Aucune authentification)

L'API publique permet un accès non authentifié aux points de terminaison publics :

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

    // Récupérer les commentaires via PublicAPI
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

#### API par défaut (Requiert une clé API)

L'API par défaut requiert une authentification avec votre clé API :

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

    // Créer un contexte authentifié avec la clé API
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // Récupérer les commentaires en utilisant DefaultAPI authentifié
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

#### API de modération (Tableau de bord du modérateur)

L'API de modération alimente le tableau de bord du modérateur. Elle fournit des méthodes pour lister,
compter, rechercher et exporter des commentaires, actions de modération (supprimer/restaurer,
signaler, définir l'état revue/spam/approuvé, votes, rouvrir/fermer les fils), bannissements (interdire de
commenter, annuler, résumés pré-bannissement, état et préférences du bannissement, nombre d'utilisateurs bannis),
et badges & confiance (attribuer/retirer des badges, badges manuels, obtenir/définir le facteur de confiance, profil
interne de l'utilisateur). Toutes les méthodes de modération acceptent un `sso` paramètre pour
les modérateurs authentifiés via SSO:

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

    // Lister les commentaires pour modération en utilisant ModerationAPI
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