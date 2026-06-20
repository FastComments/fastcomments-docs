## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Nie |  |
| byIPFromComment | string | query | Nie |  |
| filter | string | query | Nie |  |
| searchFilters | string | query | Nie |  |
| demo | boolean | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_count_comments_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład GetCount'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	filter := "filter_example" // string |  (opcjonalne)
	searchFilters := "searchFilters_example" // string |  (opcjonalne)
	demo := true // bool |  (opcjonalne)
	sso := "sso_example" // string |  (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetCount(context.Background()).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filter(filter).SearchFilters(searchFilters).Demo(demo).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `GetCount`: ModerationAPICountCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetCount`: %v\n", resp)
}
[inline-code-end]