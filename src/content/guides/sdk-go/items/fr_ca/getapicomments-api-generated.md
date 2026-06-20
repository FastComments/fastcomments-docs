## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| page | number | query | Non |  |
| count | number | query | Non |  |
| text-search | string | query | Non |  |
| byIPFromComment | string | query | Non |  |
| filters | string | query | Non |  |
| searchFilters | string | query | Non |  |
| sorts | string | query | Non |  |
| demo | boolean | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_get_comments_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple GetApiComments'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	page := float64(1.2) // float64 |  (facultatif)
	count := float64(1.2) // float64 |  (facultatif)
	textSearch := "textSearch_example" // string |  (facultatif)
	byIPFromComment := "byIPFromComment_example" // string |  (facultatif)
	filters := "filters_example" // string |  (facultatif)
	searchFilters := "searchFilters_example" // string |  (facultatif)
	sorts := "sorts_example" // string |  (facultatif)
	demo := true // bool |  (facultatif)
	sso := "sso_example" // string |  (facultatif)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetApiComments(context.Background()).Page(page).Count(count).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filters(filters).SearchFilters(searchFilters).Sorts(sorts).Demo(demo).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetApiComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `GetApiComments`: ModerationAPIGetCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetApiComments`: %v\n", resp)
}
[inline-code-end]

---