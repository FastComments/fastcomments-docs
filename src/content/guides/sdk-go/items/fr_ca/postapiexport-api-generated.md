## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|-------------|------------|-------------|
| tenantId | string | requête | Oui |  |
| text-search | string | requête | Non |  |
| byIPFromComment | string | requête | Non |  |
| filters | string | requête | Non |  |
| searchFilters | string | requête | Non |  |
| sorts | string | requête | Non |  |
| sso | string | requête | Non |  |

## Réponse

Retourne : [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_export_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple PostApiExport'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sorts := "sorts_example" // string |  (facultatif)
	sso := "sso_example" // string |  (facultatif)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostApiExport(context.Background()).TenantId(tenantId).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filters(filters).SearchFilters(searchFilters).Sorts(sorts).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostApiExport``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `PostApiExport` : ModerationExportResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostApiExport`: %v\n", resp)
}
[inline-code-end]