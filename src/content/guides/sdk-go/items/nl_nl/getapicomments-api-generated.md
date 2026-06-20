## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| page | number | query | Nee |  |
| count | number | query | Nee |  |
| text-search | string | query | Nee |  |
| byIPFromComment | string | query | Nee |  |
| filters | string | query | Nee |  |
| searchFilters | string | query | Nee |  |
| sorts | string | query | Nee |  |
| demo | boolean | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_get_comments_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'GetApiComments Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	page := float64(1.2) // float64 |  (optioneel)
	count := float64(1.2) // float64 |  (optioneel)
	textSearch := "textSearch_example" // string |  (optioneel)
	byIPFromComment := "byIPFromComment_example" // string |  (optioneel)
	filters := "filters_example" // string |  (optioneel)
	searchFilters := "searchFilters_example" // string |  (optioneel)
	sorts := "sorts_example" // string |  (optioneel)
	demo := true // bool |  (optioneel)
	sso := "sso_example" // string |  (optioneel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetApiComments(context.Background()).Page(page).Count(count).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filters(filters).SearchFilters(searchFilters).Sorts(sorts).Demo(demo).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetApiComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// antwoord van `GetApiComments`: ModerationAPIGetCommentsResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetApiComments`: %v\n", resp)
}
[inline-code-end]