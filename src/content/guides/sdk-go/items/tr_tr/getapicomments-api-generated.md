## Parametreler

| Name | Type | Location | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
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
	page := float64(1.2) // float64 |  (isteğe bağlı)
	count := float64(1.2) // float64 |  (isteğe bağlı)
	textSearch := "textSearch_example" // string |  (isteğe bağlı)
	byIPFromComment := "byIPFromComment_example" // string |  (isteğe bağlı)
	filters := "filters_example" // string |  (isteğe bağlı)
	searchFilters := "searchFilters_example" // string |  (isteğe bağlı)
	sorts := "sorts_example" // string |  (isteğe bağlı)
	demo := true // bool |  (isteğe bağlı)
	sso := "sso_example" // string |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetApiComments(context.Background()).Page(page).Count(count).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filters(filters).SearchFilters(searchFilters).Sorts(sorts).Demo(demo).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetApiComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetApiComments`: ModerationAPIGetCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetApiComments`: %v\n", resp)
}
[inline-code-end]