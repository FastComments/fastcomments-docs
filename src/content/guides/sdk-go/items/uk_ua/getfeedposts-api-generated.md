req
tenantId
afterId

## Параметри

| Назва | Type | Location | Required | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| afterId | string | query | Ні |  |
| limit | integer | query | Ні |  |
| tags | array | query | Ні |  |

## Відповідь

Повертає: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_feed_posts_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад GetFeedPosts'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterId := "afterId_example" // string |  (необов'язковий)
	limit := int32(56) // int32 |  (необов'язковий)
	tags := []string{"Inner_example"} // []string |  (необов'язковий)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetFeedPosts(context.Background()).TenantId(tenantId).AfterId(afterId).Limit(limit).Tags(tags).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetFeedPosts``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `GetFeedPosts`: GetFeedPostsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetFeedPosts`: %v\n", resp)
}
[inline-code-end]