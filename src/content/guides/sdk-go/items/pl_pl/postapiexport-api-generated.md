## Parametry

| Name | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| text-search | string | query | Nie |  |
| byIPFromComment | string | query | Nie |  |
| filters | string | query | Nie |  |
| searchFilters | string | query | Nie |  |
| sorts | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_export_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład PostApiExport'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	textSearch := "textSearch_example" // string |  (opcjonalne)
	byIPFromComment := "byIPFromComment_example" // string |  (opcjonalne)
	filters := "filters_example" // string |  (opcjonalne)
	searchFilters := "searchFilters_example" // string |  (opcjonalne)
	sorts := "sorts_example" // string |  (opcjonalne)
	sso := "sso_example" // string |  (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostApiExport(context.Background()).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filters(filters).SearchFilters(searchFilters).Sorts(sorts).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostApiExport``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `PostApiExport`: ModerationExportResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostApiExport`: %v\n", resp)
}
[inline-code-end]