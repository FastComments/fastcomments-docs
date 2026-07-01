## Paramètres

| Nom | Type | Location | Obligatoire | Description |
|------|------|----------|-------------|-------------|
| tenantId | string | query | Oui |  |
| text-search | string | query | Non |  |
| byIPFromComment | string | query | Non |  |
| filter | string | query | Non |  |
| searchFilters | string | query | Non |  |
| demo | boolean | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_count_comments_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple GetCount'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	textSearch := "textSearch_example" // string |  (optionnel)
	byIPFromComment := "byIPFromComment_example" // string |  (optionnel)
	filter := "filter_example" // string |  (optionnel)
	searchFilters := "searchFilters_example" // string |  (optionnel)
	demo := true // bool |  (optionnel)
	sso := "sso_example" // string |  (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetCount(context.Background()).TenantId(tenantId).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filter(filter).SearchFilters(searchFilters).Demo(demo).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Erreur lors de l'appel de `ModerationAPI.GetCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Réponse HTTP complète : %v\n", r)
	}
	// réponse de `GetCount`: ModerationAPICountCommentsResponse
	fmt.Fprintf(os.Stdout, "Réponse de `ModerationAPI.GetCount` : %v\n", resp)
}
[inline-code-end]