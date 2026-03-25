## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| usernameStartsWith | string | query | Nee |  |
| mentionGroupIds | array | query | Nee |  |
| sso | string | query | Nee |  |
| searchSection | string | query | Nee |  |

## Response

Retourneert: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_search_users_200_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'SearchUsers Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	urlId := "urlId_example" // string | 
	usernameStartsWith := "usernameStartsWith_example" // string |  (optioneel)
	mentionGroupIds := []string{"Inner_example"} // []string |  (optioneel)
	sso := "sso_example" // string |  (optioneel)
	searchSection := "searchSection_example" // string |  (optioneel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.SearchUsers(context.Background(), tenantId).UrlId(urlId).UsernameStartsWith(usernameStartsWith).MentionGroupIds(mentionGroupIds).Sso(sso).SearchSection(searchSection).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.SearchUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// reactie van `SearchUsers`: SearchUsers200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.SearchUsers`: %v\n", resp)
}
[inline-code-end]