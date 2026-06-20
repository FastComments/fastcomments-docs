## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| value | string | query | Nee |  |
| filters | string | query | Nee |  |
| searchFilters | string | query | Nee |  |
| sso | string | query | Nee |  |

## Response

Retourneert: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_comment_search_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'GetSearchCommentsSummary Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	value := "value_example" // string |  (optioneel)
	filters := "filters_example" // string |  (optioneel)
	searchFilters := "searchFilters_example" // string |  (optioneel)
	sso := "sso_example" // string |  (optioneel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetSearchCommentsSummary(context.Background()).Value(value).Filters(filters).SearchFilters(searchFilters).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetSearchCommentsSummary``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// antwoord van `GetSearchCommentsSummary`: ModerationCommentSearchResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetSearchCommentsSummary`: %v\n", resp)
}
[inline-code-end]

---