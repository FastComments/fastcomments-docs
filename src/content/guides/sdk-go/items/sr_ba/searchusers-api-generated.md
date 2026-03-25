## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| usernameStartsWith | string | query | Не |  |
| mentionGroupIds | array | query | Не |  |
| sso | string | query | Не |  |
| searchSection | string | query | Не |  |

## Одговор

Враћа: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_search_users_200_response.go)

## Пример

[inline-code-attrs-start title = 'Пример за SearchUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	usernameStartsWith := "usernameStartsWith_example" // string |  (необавезно)
	mentionGroupIds := []string{"Inner_example"} // []string |  (необавезно)
	sso := "sso_example" // string |  (необавезно)
	searchSection := "searchSection_example" // string |  (необавезно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.SearchUsers(context.Background(), tenantId).UrlId(urlId).UsernameStartsWith(usernameStartsWith).MentionGroupIds(mentionGroupIds).Sso(sso).SearchSection(searchSection).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.SearchUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// одговор од `SearchUsers`: SearchUsers200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.SearchUsers`: %v\n", resp)
}
[inline-code-end]

---