## Parameters

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| text-search | string | query | Non |  |
| byIPFromComment | string | query | Non |  |
| filters | string | query | Non |  |
| searchFilters | string | query | Non |  |
| afterId | string | query | Non |  |
| demo | boolean | query | Non |  |
| sso | string | query | Non |  |

## Response

Retourne : [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_get_comment_ids_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple GetApiIds'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	textSearch := "textSearch_example" // string |  (facultatif)
	byIPFromComment := "byIPFromComment_example" // string |  (facultatif)
	filters := "filters_example" // string |  (facultatif)
	searchFilters := "searchFilters_example" // string |  (facultatif)
	afterId := "afterId_example" // string |  (facultatif)
	demo := true // bool |  (facultatif)
	sso := "sso_example" // string |  (facultatif)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetApiIds(context.Background()).TenantId(tenantId).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filters(filters).SearchFilters(searchFilters).AfterId(afterId).Demo(demo).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Erreur lors de l'appel `ModerationAPI.GetApiIds``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Réponse HTTP complète : %v\n", r)
	}
	// réponse de `GetApiIds` : ModerationAPIGetCommentIdsResponse
	fmt.Fprintf(os.Stdout, "Réponse de `ModerationAPI.GetApiIds` : %v\n", resp)
}
[inline-code-end]