## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| text-search | string | 查询 | 否 |  |
| byIPFromComment | string | 查询 | 否 |  |
| filters | string | 查询 | 否 |  |
| searchFilters | string | 查询 | 否 |  |
| afterId | string | 查询 | 否 |  |
| demo | boolean | 查询 | 否 |  |
| sso | string | 查询 | 否 |  |

## 响应

返回: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_get_comment_ids_response.go)

## 示例

[inline-code-attrs-start title = 'GetApiIds 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	textSearch := "textSearch_example" // string |  （可选）
	byIPFromComment := "byIPFromComment_example" // string |  （可选）
	filters := "filters_example" // string |  （可选）
	searchFilters := "searchFilters_example" // string |  （可选）
	afterId := "afterId_example" // string |  （可选）
	demo := true // bool |  （可选）
	sso := "sso_example" // string |  （可选）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetApiIds(context.Background()).TextSearch(textSearch).ByIPFromComment(byIPFromComment).Filters(filters).SearchFilters(searchFilters).AfterId(afterId).Demo(demo).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetApiIds``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `GetApiIds` 的响应: ModerationAPIGetCommentIdsResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetApiIds`: %v\n", resp)
}
[inline-code-end]

---