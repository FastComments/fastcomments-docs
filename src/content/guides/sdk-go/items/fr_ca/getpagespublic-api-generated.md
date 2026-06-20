Liste les pages pour un locataire. Utilisé par le client de bureau FChat pour remplir sa liste de salons.
Nécessite `enableFChat` à true dans la configuration personnalisée résolue pour chaque page.
Les pages nécessitant SSO sont filtrées en fonction des accès de groupe de l'utilisateur demandeur.

## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| cursor | string | query | Non | Curseur de pagination opaque renvoyé comme `nextCursor` à partir d'une requête précédente. Lié au même `sortBy`. |
| limit | integer | query | Non | 1..200, par défaut 50 |
| q | string | query | Non | Filtre optionnel sur le préfixe du titre, insensible à la casse. |
| sortBy | string | query | Non | Ordre de tri. `updatedAt` (par défaut, les plus récents en premier), `commentCount` (les plus commentés en premier), ou `title` (alphabétique). |
| hasComments | boolean | query | Non | Si true, ne retourner que les pages ayant au moins un commentaire. |

## Réponse

Renvoie: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

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
	cursor := "cursor_example" // string | Curseur opaque de pagination renvoyé comme `nextCursor` à partir d'une requête précédente. Lié au même `sortBy`. (optionnel)
	limit := int32(56) // int32 | 1..200, par défaut 50 (optionnel)
	q := "q_example" // string | Filtre optionnel sur le préfixe du titre, insensible à la casse. (optionnel)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | Ordre de tri. `updatedAt` (par défaut, les plus récents en premier), `commentCount` (les plus commentés en premier), ou `title` (alphabétique). (optionnel)
	hasComments := true // bool | Si true, ne retourner que les pages ayant au moins un commentaire. (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetPagesPublic(context.Background(), tenantId).Cursor(cursor).Limit(limit).Q(q).SortBy(sortBy).HasComments(hasComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetPagesPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `GetPagesPublic`: GetPublicPagesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetPagesPublic`: %v\n", resp)
}
[inline-code-end]