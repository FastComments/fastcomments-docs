## Parametri

| Nome | Tipo | Posizione | Richiesto | Descrizione |
|------|------|-----------|-----------|-------------|
| tenantId | string | query | Sì |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| afterId | string | query | No |  |
| demo | boolean | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_get_comment_ids_response.go)

## Esempio

[inline-code-attrs-start title = 'GetApiIds Esempio'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	textSearch := "textSearch_example" // string |  (opzionale)
	byIPFromComment := "byIPFromComment_example" // string |  (opzionale)
	filters := "filters_example" // string |  (opzionale)
	searchFilters := "searchFilters_example" // string |  (opzionale)
	afterId := "afterId_example" // string |  (opzionale)
	demo := true // bool |  (opzionale)
	sso := "sso_example" // string |  (opzionale)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetApiIds(context.Background()).TenantId(tenantId).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filters(filters).SearchFilters(searchFilters).AfterId(afterId).Demo(demo).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Errore durante la chiamata `ModerationAPI.GetApiIds``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Risposta HTTP completa: %v\n", r)
	}
	// risposta da `GetApiIds`: ModerationAPIGetCommentIdsResponse
	fmt.Fprintf(os.Stdout, "Risposta da `ModerationAPI.GetApiIds`: %v\n", resp)
}
[inline-code-end]