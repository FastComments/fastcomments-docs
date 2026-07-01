## 參數

| 名稱 | 型別 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| page | number | query | No |  |
| count | number | query | No |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sorts | string | query | No |  |
| demo | boolean | query | No |  |
| sso | string | query | No |  |

## 回應

Returns: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_get_comments_response.go)

## 範例

[inline-code-attrs-start title = 'GetApiComments 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := float64(1.2) // float64 |  (可選)
	count := float64(1.2) // float64 |  (可選)
	textSearch := "textSearch_example" // string |  (可選)
	byIPFromComment := "byIPFromComment_example" // string |  (可選)
	filters := "filters_example" // string |  (可選)
	searchFilters := "searchFilters_example" // string |  (可選)
	sorts := "sorts_example" // string |  (可選)
	demo := true // bool |  (可選)
	sso := "sso_example" // string |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetApiComments(context.Background()).TenantId(tenantId).Page(page).Count(count).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filters(filters).SearchFilters(searchFilters).Sorts(sorts).Demo(demo).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "呼叫 `ModerationAPI.GetApiComments` 時發生錯誤: %v\n", err)
		fmt.Fprintf(os.Stderr, "完整 HTTP 回應: %v\n", r)
	}
	// `GetApiComments` 的回應: ModerationAPIGetCommentsResponse
	fmt.Fprintf(os.Stdout, "`ModerationAPI.GetApiComments` 的回應: %v\n", resp)
}
[inline-code-end]