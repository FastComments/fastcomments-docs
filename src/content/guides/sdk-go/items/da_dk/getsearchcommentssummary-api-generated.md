## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | Nej |  |
| filters | string | query | Nej |  |
| searchFilters | string | query | Nej |  |
| sso | string | query | Nej |  |

## Response

Returnerer: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_comment_search_response.go)

## Eksempel

[inline-code-attrs-start title = 'GetSearchCommentsSummary Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	value := "value_example" // string |  (valgfri)
	filters := "filters_example" // string |  (valgfri)
	searchFilters := "searchFilters_example" // string |  (valgfri)
	sso := "sso_example" // string |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetSearchCommentsSummary(context.Background()).Value(value).Filters(filters).SearchFilters(searchFilters).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetSearchCommentsSummary``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `GetSearchCommentsSummary`: ModerationCommentSearchResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetSearchCommentsSummary`: %v\n", resp)
}
[inline-code-end]