req
tenantId
afterId

## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| afterId | string | query | 否 |  |
| limit | integer | query | 否 |  |
| tags | array | query | 否 |  |
| sso | string | query | 否 |  |
| isCrawler | boolean | query | 否 |  |
| includeUserInfo | boolean | query | 否 |  |

## 响应

返回：[`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_feed_posts_public_200_response.go)

## 示例

[inline-code-attrs-start title = 'GetFeedPostsPublic 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	afterId := "afterId_example" // string |  （可选）
	limit := int32(56) // int32 |  （可选）
	tags := []string{"Inner_example"} // []string |  （可选）
	sso := "sso_example" // string |  （可选）
	isCrawler := true // bool |  （可选）
	includeUserInfo := true // bool |  （可选）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetFeedPostsPublic(context.Background(), tenantId).AfterId(afterId).Limit(limit).Tags(tags).Sso(sso).IsCrawler(isCrawler).IncludeUserInfo(includeUserInfo).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetFeedPostsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `GetFeedPostsPublic` 的响应: GetFeedPostsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetFeedPostsPublic`: %v\n", resp)
}
[inline-code-end]