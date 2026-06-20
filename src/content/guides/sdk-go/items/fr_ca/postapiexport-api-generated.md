## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Non |  |
| byIPFromComment | string | query | Non |  |
| filters | string | query | Non |  |
| searchFilters | string | query | Non |  |
| sorts | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_export_response.go)

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
	textSearch := "textSearch_example" // string |  (optionnel)
	byIPFromComment := "byIPFromComment_example" // string |  (optionnel)
	filters := "filters_example" // string |  (optionnel)
	searchFilters := "searchFilters_example" // string |  (optionnel)
	sorts := "sorts_example" // string |  (optionnel)
	sso := "sso_example" // string |  (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostApiExport(context.Background()).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filters(filters).SearchFilters(searchFilters).Sorts(sorts).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostApiExport``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `PostApiExport` : ModerationExportResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostApiExport`: %v\n", resp)
}
[inline-code-end]