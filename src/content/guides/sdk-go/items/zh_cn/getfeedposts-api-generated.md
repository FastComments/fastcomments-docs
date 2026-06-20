请求
tenantId
afterId

## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| afterId | string | query | 否 |  |
| limit | integer | query | 否 |  |
| tags | array | query | 否 |  |

## 响应

返回: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_feed_posts_response.go)

## 示例

[inline-code-attrs-start title = 'GetFeedPosts 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterId := "afterId_example" // string |  (可选)
	limit := int32(56) // int32 |  (可选)
	tags := []string{"Inner_example"} // []string |  (可选)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetFeedPosts(context.Background()).TenantId(tenantId).AfterId(afterId).Limit(limit).Tags(tags).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetFeedPosts``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `GetFeedPosts` 的响应: GetFeedPostsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetFeedPosts`: %v\n", resp)
}
[inline-code-end]