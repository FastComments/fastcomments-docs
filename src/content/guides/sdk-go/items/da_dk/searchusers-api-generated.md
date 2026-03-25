## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| usernameStartsWith | string | query | Nej |  |
| mentionGroupIds | array | query | Nej |  |
| sso | string | query | Nej |  |
| searchSection | string | query | Nej |  |

## Svar

Returnerer: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_search_users_200_response.go)

## Eksempel

[inline-code-attrs-start title = 'SearchUsers Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	usernameStartsWith := "usernameStartsWith_example" // string |  (valgfrit)
	mentionGroupIds := []string{"Inner_example"} // []string |  (valgfrit)
	sso := "sso_example" // string |  (valgfrit)
	searchSection := "searchSection_example" // string |  (valgfrit)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.SearchUsers(context.Background(), tenantId).UrlId(urlId).UsernameStartsWith(usernameStartsWith).MentionGroupIds(mentionGroupIds).Sso(sso).SearchSection(searchSection).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.SearchUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `SearchUsers`: SearchUsers200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.SearchUsers`: %v\n", resp)
}
[inline-code-end]