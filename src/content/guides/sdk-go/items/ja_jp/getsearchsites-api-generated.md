## パラメータ

| 名前 | 型 | ロケーション | 必須 | 説明 |
|------|------|----------|----------|-------------|
| value | string | query | No |  |
| sso | string | query | No |  |

## レスポンス

戻り値: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_site_search_response.go)

## 例

[inline-code-attrs-start title = 'GetSearchSites の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	value := "value_example" // string |  (任意)
	sso := "sso_example" // string |  (任意)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetSearchSites(context.Background()).Value(value).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetSearchSites``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetSearchSites` のレスポンス: ModerationSiteSearchResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetSearchSites`: %v\n", resp)
}
[inline-code-end]