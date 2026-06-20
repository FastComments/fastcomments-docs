---
Auteurs de commentaires précédents sur la page qui ne sont pas actuellement en ligne. Triés par displayName.
Utilisez ceci après avoir épuisé /users/online pour afficher une section « Membres ».
Pagination par curseur sur commenterName : le serveur parcourt l'index partiel {tenantId, urlId, commenterName} à partir de afterName vers l'avant via $gt, sans coût de $skip.

## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifiant d'URL de la page (nettoyé côté serveur). |
| afterName | string | query | No | Curseur : transmettre nextAfterName de la réponse précédente. |
| afterUserId | string | query | No | Désambiguïsateur du curseur : transmettre nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom n'entraînent pas la suppression d'entrées. |

## Réponse

Renvoie : [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple GetOfflineUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterName := "afterName_example" // string | Curseur : transmettre nextAfterName de la réponse précédente. (optionnel)
	afterUserId := "afterUserId_example" // string | Désambiguïsateur du curseur : transmettre nextAfterUserId de la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom n'entraînent pas la suppression d'entrées. (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetOfflineUsers`: PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]

---