## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| text-search | string | query | Ne |  |
| byIPFromComment | string | query | Ne |  |
| filter | string | query | Ne |  |
| searchFilters | string | query | Ne |  |
| demo | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Returns: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_count_comments_response.go)

## Primer

[inline-code-attrs-start title = 'Primer GetCount'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	textSearch := "textSearch_example" // string |  (neobvezno)
	byIPFromComment := "byIPFromComment_example" // string |  (neobvezno)
	filter := "filter_example" // string |  (neobvezno)
	searchFilters := "searchFilters_example" // string |  (neobvezno)
	demo := true // bool |  (neobvezno)
	sso := "sso_example" // string |  (neobvezno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetCount(context.Background()).TenantId(tenantId).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filter(filter).SearchFilters(searchFilters).Demo(demo).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Napaka pri klicu `ModerationAPI.GetCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Poln HTTP odgovor: %v\n", r)
	}
	// odziv iz `GetCount`: ModerationAPICountCommentsResponse
	fmt.Fprintf(os.Stdout, "Odziv iz `ModerationAPI.GetCount`: %v\n", resp)
}
[inline-code-end]