## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| postIds | array | query | 是 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`GetFeedPostsStats200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_feed_posts_stats_200_response.go)

## 示例

[inline-code-attrs-start title = 'GetFeedPostsStats 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	postIds := []string{"Inner_example"} // []string | 
	sso := "sso_example" // string |  (可选)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetFeedPostsStats(context.Background(), tenantId).PostIds(postIds).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetFeedPostsStats``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `GetFeedPostsStats` 的响应：GetFeedPostsStats200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetFeedPostsStats`: %v\n", resp)
}
[inline-code-end]

---