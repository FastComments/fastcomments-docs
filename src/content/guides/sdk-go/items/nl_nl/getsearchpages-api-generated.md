## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| value | string | query | No |  |
| sso | string | query | No |  |

## Antwoord

Retourneert: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_page_search_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'GetSearchPages Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (optioneel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetSearchPages(context.Background()).Value(value).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetSearchPages``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// antwoord van `GetSearchPages`: ModerationPageSearchResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetSearchPages`: %v\n", resp)
}
[inline-code-end]