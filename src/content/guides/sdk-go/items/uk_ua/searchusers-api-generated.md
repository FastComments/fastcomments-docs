## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так |  |
| usernameStartsWith | string | query | Ні |  |
| mentionGroupIds | array | query | Ні |  |
| sso | string | query | Ні |  |
| searchSection | string | query | Ні |  |

## Відповідь

Повертає: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_search_users_200_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад SearchUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	usernameStartsWith := "usernameStartsWith_example" // string |  (необов'язково)
	mentionGroupIds := []string{"Inner_example"} // []string |  (необов'язково)
	sso := "sso_example" // string |  (необов'язково)
	searchSection := "searchSection_example" // string |  (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.SearchUsers(context.Background(), tenantId).UrlId(urlId).UsernameStartsWith(usernameStartsWith).MentionGroupIds(mentionGroupIds).Sso(sso).SearchSection(searchSection).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.SearchUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `SearchUsers`: SearchUsers200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.SearchUsers`: %v\n", resp)
}
[inline-code-end]

---