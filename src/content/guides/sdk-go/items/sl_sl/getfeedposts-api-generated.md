req
tenantId
afterId

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| afterId | string | query | Ne |  |
| limit | integer | query | Ne |  |
| tags | array | query | Ne |  |

## Odgovor

Vrne: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_feed_posts_response.go)

## Primer

[inline-code-attrs-start title = 'Primer GetFeedPosts'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterId := "afterId_example" // string |  (izbirno)
	limit := int32(56) // int32 |  (izbirno)
	tags := []string{"Inner_example"} // []string |  (izbirno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetFeedPosts(context.Background()).TenantId(tenantId).AfterId(afterId).Limit(limit).Tags(tags).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetFeedPosts``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor iz `GetFeedPosts`: GetFeedPostsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetFeedPosts`: %v\n", resp)
}
[inline-code-end]