## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | 否 |  |
| filters | string | query | 否 |  |
| searchFilters | string | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_comment_search_response.go)

## 示例

[inline-code-attrs-start title = 'GetSearchCommentsSummary 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	value := "value_example" // string |  （可选）
	filters := "filters_example" // string |  （可选）
	searchFilters := "searchFilters_example" // string |  （可选）
	sso := "sso_example" // string |  （可选）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetSearchCommentsSummary(context.Background()).Value(value).Filters(filters).SearchFilters(searchFilters).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetSearchCommentsSummary``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `GetSearchCommentsSummary` 的响应: ModerationCommentSearchResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetSearchCommentsSummary`: %v\n", resp)
}
[inline-code-end]

---