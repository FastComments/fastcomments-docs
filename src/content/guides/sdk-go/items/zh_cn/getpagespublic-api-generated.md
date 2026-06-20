列出租户的页面。由 FChat 桌面客户端用于填充其房间列表。要求在每个页面的已解析自定义配置中 `enableFChat` 为 true。需要 SSO 的页面将根据请求用户的组访问权限进行过滤。

## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| cursor | string | query | 否 | 不透明的分页游标，由先前请求返回为 `nextCursor`。与相同的 `sortBy` 绑定。 |
| limit | integer | query | 否 | 1..200，默认 50 |
| q | string | query | 否 | 可选的、大小写不敏感的标题前缀过滤。 |
| sortBy | string | query | 否 | 排序顺序。`updatedAt`（默认，按最新优先），`commentCount`（按评论数最多优先），或 `title`（按字母顺序）。 |
| hasComments | boolean | query | 否 | 如果为 true，则只返回至少有一条评论的页面。 |

## 响应

返回: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

## 示例

[inline-code-attrs-start title = 'GetPagesPublic 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	cursor := "cursor_example" // string | 不透明的分页游标，由先前请求返回为 `nextCursor`。与相同的 `sortBy` 绑定。 (optional)
	limit := int32(56) // int32 | 1..200，默认 50 (optional)
	q := "q_example" // string | 可选的、大小写不敏感的标题前缀过滤。 (optional)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | 排序顺序。`updatedAt`（默认，按最新优先），`commentCount`（按评论数最多优先），或 `title`（按字母顺序）。 (optional)
	hasComments := true // bool | 如果为 true，则只返回至少有一条评论的页面。 (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetPagesPublic(context.Background(), tenantId).Cursor(cursor).Limit(limit).Q(q).SortBy(sortBy).HasComments(hasComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetPagesPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetPagesPublic`: GetPublicPagesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetPagesPublic`: %v\n", resp)
}
[inline-code-end]