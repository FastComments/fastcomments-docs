## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい |  |
| usernameStartsWith | string | query | いいえ |  |
| mentionGroupIds | array | query | いいえ |  |
| sso | string | query | いいえ |  |
| searchSection | string | query | いいえ |  |

## レスポンス

戻り値: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_search_users_200_response.go)

## 例

[inline-code-attrs-start title = 'SearchUsers の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	usernameStartsWith := "usernameStartsWith_example" // string |  (省略可能)
	mentionGroupIds := []string{"Inner_example"} // []string |  (省略可能)
	sso := "sso_example" // string |  (省略可能)
	searchSection := "searchSection_example" // string |  (省略可能)

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