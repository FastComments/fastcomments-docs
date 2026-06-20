## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Nej |  |
| byIPFromComment | string | query | Nej |  |
| filter | string | query | Nej |  |
| searchFilters | string | query | Nej |  |
| demo | boolean | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_count_comments_response.go)

## Eksempel

[inline-code-attrs-start title = 'GetCount Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	textSearch := "textSearch_example" // string |  (valgfri)
	byIPFromComment := "byIPFromComment_example" // string |  (valgfri)
	filter := "filter_example" // string |  (valgfri)
	searchFilters := "searchFilters_example" // string |  (valgfri)
	demo := true // bool |  (valgfri)
	sso := "sso_example" // string |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetCount(context.Background()).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filter(filter).SearchFilters(searchFilters).Demo(demo).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Svar fra `GetCount`: ModerationAPICountCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetCount`: %v\n", resp)
}
[inline-code-end]