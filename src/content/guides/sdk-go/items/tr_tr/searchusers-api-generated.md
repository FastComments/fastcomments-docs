## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet |  |
| usernameStartsWith | string | query | Hayır |  |
| mentionGroupIds | array | query | Hayır |  |
| sso | string | query | Hayır |  |
| searchSection | string | query | Hayır |  |

## Yanıt

Döndürür: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_search_users_200_response.go)

## Örnek

[inline-code-attrs-start title = 'SearchUsers Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	usernameStartsWith := "usernameStartsWith_example" // string |  (isteğe bağlı)
	mentionGroupIds := []string{"Inner_example"} // []string |  (isteğe bağlı)
	sso := "sso_example" // string |  (isteğe bağlı)
	searchSection := "searchSection_example" // string |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.SearchUsers(context.Background(), tenantId).UrlId(urlId).UsernameStartsWith(usernameStartsWith).MentionGroupIds(mentionGroupIds).Sso(sso).SearchSection(searchSection).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.SearchUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `SearchUsers`'ten dönen yanıt: SearchUsers200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.SearchUsers`: %v\n", resp)
}
[inline-code-end]