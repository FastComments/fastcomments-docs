## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| page | number | query | Nein |  |
| count | number | query | Nein |  |
| text-search | string | query | Nein |  |
| byIPFromComment | string | query | Nein |  |
| filters | string | query | Nein |  |
| searchFilters | string | query | Nein |  |
| sorts | string | query | Nein |  |
| demo | boolean | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_get_comments_response.go)

## Beispiel

[inline-code-attrs-start title = 'GetApiComments Beispiel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	page := float64(1.2) // float64 |  (optional)
	count := float64(1.2) // float64 |  (optional)
	textSearch := "textSearch_example" // string |  (optional)
	byIPFromComment := "byIPFromComment_example" // string |  (optional)
	filters := "filters_example" // string |  (optional)
	searchFilters := "searchFilters_example" // string |  (optional)
	sorts := "sorts_example" // string |  (optional)
	demo := true // bool |  (optional)
	sso := "sso_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetApiComments(context.Background()).Page(page).Count(count).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filters(filters).SearchFilters(searchFilters).Sorts(sorts).Demo(demo).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetApiComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `GetApiComments`: ModerationAPIGetCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetApiComments`: %v\n", resp)
}
[inline-code-end]