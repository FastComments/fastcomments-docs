## Parametreler

| Ad | Tür | Yer | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| page | number | query | Hayır |  |
| count | number | query | Hayır |  |
| text-search | string | query | Hayır |  |
| byIPFromComment | string | query | Hayır |  |
| filters | string | query | Hayır |  |
| searchFilters | string | query | Hayır |  |
| sorts | string | query | Hayır |  |
| demo | boolean | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_get_comments_response.go)

## Örnek

[inline-code-attrs-start title = 'GetApiComments Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := float64(1.2) // float64 |  (opsiyonel)
	count := float64(1.2) // float64 |  (opsiyonel)
	textSearch := "textSearch_example" // string |  (opsiyonel)
	byIPFromComment := "byIPFromComment_example" // string |  (opsiyonel)
	filters := "filters_example" // string |  (opsiyonel)
	searchFilters := "searchFilters_example" // string |  (opsiyonel)
	sorts := "sorts_example" // string |  (opsiyonel)
	demo := true // bool |  (opsiyonel)
	sso := "sso_example" // string |  (opsiyonel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetApiComments(context.Background()).TenantId(tenantId).Page(page).Count(count).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filters(filters).SearchFilters(searchFilters).Sorts(sorts).Demo(demo).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "`ModerationAPI.GetApiComments` çağrısı sırasında hata: %v\n", err)
		fmt.Fprintf(os.Stderr, "Tam HTTP yanıtı: %v\n", r)
	}
	// `GetApiComments`'den gelen yanıt: ModerationAPIGetCommentsResponse
	fmt.Fprintf(os.Stdout, "`ModerationAPI.GetApiComments`'ten gelen yanıt: %v\n", resp)
}
[inline-code-end]