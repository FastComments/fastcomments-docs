## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| text-search | string | query | Ne |  |
| byIPFromComment | string | query | Ne |  |
| filters | string | query | Ne |  |
| searchFilters | string | query | Ne |  |
| sorts | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_export_response.go)

## Primjer

[inline-code-attrs-start title = 'PostApiExport Primjer'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	textSearch := "textSearch_example" // string |  (opcionalno)
	byIPFromComment := "byIPFromComment_example" // string |  (opcionalno)
	filters := "filters_example" // string |  (opcionalno)
	searchFilters := "searchFilters_example" // string |  (opcionalno)
	sorts := "sorts_example" // string |  (opcionalno)
	sso := "sso_example" // string |  (opcionalno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostApiExport(context.Background()).TenantId(tenantId).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filters(filters).SearchFilters(searchFilters).Sorts(sorts).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Greška prilikom pozivanja `ModerationAPI.PostApiExport``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Pun HTTP odgovor: %v\n", r)
	}
	// odgovor od `PostApiExport`: ModerationExportResponse
	fmt.Fprintf(os.Stdout, "Odgovor od `ModerationAPI.PostApiExport`: %v\n", resp)
}
[inline-code-end]