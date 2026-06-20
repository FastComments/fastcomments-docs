Past commenters on the page who are NOT currently online. Sorted by displayName.
Utilisez ceci après avoir épuisé /users/online pour afficher une section "Membres".
Pagination par curseur sur commenterName : le serveur parcourt l'index partiel {tenantId, urlId, commenterName} depuis afterName vers l'avant via $gt, sans coût $skip.

## Parameters

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | chemin | Oui |  |
| urlId | string | paramètre de requête | Oui | Identifiant d'URL de la page (nettoyé côté serveur). |
| afterName | string | paramètre de requête | Non | Curseur : passez nextAfterName depuis la réponse précédente. |
| afterUserId | string | paramètre de requête | Non | Critère de départage du curseur : passez nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin qu'en cas d'égalité de noms, les entrées ne soient pas omises. |

## Response

Retourne : [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple de GetOfflineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Identifiant d'URL de la page (nettoyé côté serveur).
	afterName := "afterName_example" // string | Curseur : passez nextAfterName depuis la réponse précédente. (facultatif)
	afterUserId := "afterUserId_example" // string | Critère de départage du curseur : passez nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin qu'en cas d'égalité de noms, les entrées ne soient pas omises. (facultatif)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `GetOfflineUsers`: PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]

---