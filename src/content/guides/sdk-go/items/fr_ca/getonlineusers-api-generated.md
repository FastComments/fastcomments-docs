Visiteurs actuellement en ligne d'une page: personnes dont la session websocket est abonnée à la page en ce moment.
Renvoie anonCount + totalCount (abonnés de la salle, y compris les visiteurs anonymes que nous n'énumérons pas).

## Parameters

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui | Identificateur d'URL de la page (nettoyé côté serveur). |
| afterName | string | query | Non | Curseur : transmettre nextAfterName de la réponse précédente. |
| afterUserId | string | query | Non | Desempteur de curseur : transmettre nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que des égalités de nom n'entraînent pas la suppression d'entrées. |

## Réponse

Renvoie: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_online_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple GetOnlineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	urlId := "urlId_example" // string | Identificateur d'URL de la page (nettoyé côté serveur).
	afterName := "afterName_example" // string | Curseur : transmettre nextAfterName de la réponse précédente. (optionnel)
	afterUserId := "afterUserId_example" // string | Desempteur de curseur : transmettre nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que des égalités de nom n'entraînent pas la suppression d'entrées. (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOnlineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOnlineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `GetOnlineUsers`: PageUsersOnlineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOnlineUsers`: %v\n", resp)
}
[inline-code-end]