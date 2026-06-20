Liste les pages pour un tenant. Utilisé par le client de bureau FChat pour remplir sa liste de salons.
Requiert que `enableFChat` soit vrai dans la configuration personnalisée résolue pour chaque page.
Les pages qui nécessitent SSO sont filtrées en fonction de l'accès aux groupes de l'utilisateur demandeur.

## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Curseur de pagination opaque renvoyé en tant que `nextCursor` depuis une requête précédente. Lié au même `sortBy`. |
| limit | integer | query | No | 1..200, par défaut 50 |
| q | string | query | No | Filtre facultatif de préfixe de titre insensible à la casse. |
| sortBy | string | query | No | Ordre de tri. `updatedAt` (par défaut, du plus récent au plus ancien), `commentCount` (le plus de commentaires en premier), ou `title` (alphabétique). |
| hasComments | boolean | query | No | Si vrai, ne renvoyer que les pages ayant au moins un commentaire. |

## Réponse

Renvoie : [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple GetPagesPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	cursor := "cursor_example" // string | Curseur de pagination opaque renvoyé en tant que `nextCursor` depuis une requête précédente. Lié au même `sortBy`. (optionnel)
	limit := int32(56) // int32 | 1..200, valeur par défaut 50 (optionnel)
	q := "q_example" // string | Filtre facultatif par préfixe de titre insensible à la casse. (optionnel)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | Ordre de tri. `updatedAt` (par défaut, du plus récent au plus ancien), `commentCount` (le plus de commentaires en premier), ou `title` (alphabétique). (optionnel)
	hasComments := true // bool | Si vrai, ne retourner que les pages avec au moins un commentaire. (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetPagesPublic(context.Background(), tenantId).Cursor(cursor).Limit(limit).Q(q).SortBy(sortBy).HasComments(hasComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetPagesPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetPagesPublic`: GetPublicPagesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetPagesPublic`: %v\n", resp)
}
[inline-code-end]

---