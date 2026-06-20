## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | パス | 必須 |  |
| urlId | string | クエリ | 必須 |  |
| id | string | クエリ | 必須 |  |

## レスポンス

戻り値: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_v2_page_react_users_response.go)

## 例

[inline-code-attrs-start title = 'GetV2PageReactUsers の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetV2PageReactUsers(context.Background(), tenantId).UrlId(urlId).Id(id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetV2PageReactUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetV2PageReactUsers` からのレスポンス: GetV2PageReactUsersResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetV2PageReactUsers`: %v\n", resp)
}
[inline-code-end]