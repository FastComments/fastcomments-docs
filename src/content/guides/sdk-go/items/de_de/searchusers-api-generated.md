## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| usernameStartsWith | string | query | Nein |  |
| mentionGroupIds | array | query | Nein |  |
| sso | string | query | Nein |  |
| searchSection | string | query | Nein |  |

## Antwort

Gibt zurück: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_search_users_200_response.go)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für SearchUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	usernameStartsWith := "usernameStartsWith_example" // string |  (optional)
	mentionGroupIds := []string{"Inner_example"} // []string |  (optional)
	sso := "sso_example" // string |  (optional)
	searchSection := "searchSection_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.SearchUsers(context.Background(), tenantId).UrlId(urlId).UsernameStartsWith(usernameStartsWith).MentionGroupIds(mentionGroupIds).Sso(sso).SearchSection(searchSection).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.SearchUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `SearchUsers`: SearchUsers200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.SearchUsers`: %v\n", resp)
}
[inline-code-end]