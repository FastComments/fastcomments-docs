## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_site_search_response.go)

## 範例

[inline-code-attrs-start title = 'GetSearchSites 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	value := "value_example" // string |  (可選)
	sso := "sso_example" // string |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetSearchSites(context.Background()).Value(value).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetSearchSites``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetSearchSites` 的回應： ModerationSiteSearchResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetSearchSites`: %v\n", resp)
}
[inline-code-end]

---