## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filter | string | query | No |  |
| searchFilters | string | query | No |  |
| demo | boolean | query | No |  |
| sso | string | query | No |  |

## Antwort

Gibt zurück: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_count_comments_response.go)

## Beispiel

[inline-code-attrs-start title = 'GetCount-Beispiel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	textSearch := "textSearch_example" // string |  (optional)
	byIPFromComment := "byIPFromComment_example" // string |  (optional)
	filter := "filter_example" // string |  (optional)
	searchFilters := "searchFilters_example" // string |  (optional)
	demo := true // bool |  (optional)
	sso := "sso_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetCount(context.Background()).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filter(filter).SearchFilters(searchFilters).Demo(demo).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `GetCount`: ModerationAPICountCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetCount`: %v\n", resp)
}
[inline-code-end]