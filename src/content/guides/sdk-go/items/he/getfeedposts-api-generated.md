req
tenantId
afterId

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| afterId | string | query | לא |  |
| limit | integer | query | לא |  |
| tags | array | query | לא |  |

## תגובה

מחזיר: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_feed_posts_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-GetFeedPosts'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterId := "afterId_example" // string |  (אופציונלי)
	limit := int32(56) // int32 |  (אופציונלי)
	tags := []string{"Inner_example"} // []string |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetFeedPosts(context.Background()).TenantId(tenantId).AfterId(afterId).Limit(limit).Tags(tags).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetFeedPosts``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ־`GetFeedPosts`: GetFeedPostsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetFeedPosts`: %v\n", resp)
}
[inline-code-end]