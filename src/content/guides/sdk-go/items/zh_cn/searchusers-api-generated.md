## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 |  |
| usernameStartsWith | string | query | 否 |  |
| mentionGroupIds | array | query | 否 |  |
| sso | string | query | 否 |  |
| searchSection | string | query | 否 |  |

## 响应

返回: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_search_users_200_response.go)

## 示例

[inline-code-attrs-start title = 'SearchUsers 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	usernameStartsWith := "usernameStartsWith_example" // string |  (可选)
	mentionGroupIds := []string{"Inner_example"} // []string |  (可选)
	sso := "sso_example" // string |  (可选)
	searchSection := "searchSection_example" // string |  (可选)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.SearchUsers(context.Background(), tenantId).UrlId(urlId).UsernameStartsWith(usernameStartsWith).MentionGroupIds(mentionGroupIds).Sso(sso).SearchSection(searchSection).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.SearchUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `SearchUsers` 的响应: SearchUsers200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.SearchUsers`: %v\n", resp)
}
[inline-code-end]