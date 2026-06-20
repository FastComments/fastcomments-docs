```bash
go get github.com/fastcomments/fastcomments-go
```

### Utilisation du client API

#### API publique (sans authentification)

PublicAPI permet un accès non authentifié aux endpoints publics :

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

    // Récupère les commentaires en utilisant PublicAPI
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

#### API par défaut (nécessite une clé API)

DefaultAPI nécessite une authentification avec votre clé API :

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

    // Récupère les commentaires en utilisant DefaultAPI authentifié
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

#### API de modération (tableau de bord des modérateurs)

ModerationAPI alimente le tableau de bord des modérateurs. Il fournit des méthodes pour lister, compter, rechercher, et exporter des commentaires, des actions de modération (supprimer/restaurer, signaler, définir l'état révision/spam/approbation, votes, rouvrir/fermer les fils), des bannissements (interdire de commenter, annuler, résumés pré-bannissement, état et préférences de bannissement, comptes d'utilisateurs bannis), et des badges & confiance (attribuer/supprimer des badges, badges manuels, obtenir/définir le facteur de confiance, profil interne de l'utilisateur). Toutes les méthodes de modération acceptent un paramètre `sso` pour les modérateurs authentifiés via SSO :

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