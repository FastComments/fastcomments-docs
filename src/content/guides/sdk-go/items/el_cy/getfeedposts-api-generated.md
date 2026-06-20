req
tenantId
afterId

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| afterId | string | query | Όχι |  |
| limit | integer | query | Όχι |  |
| tags | array | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_feed_posts_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα GetFeedPosts'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterId := "afterId_example" // string |  (προαιρετικό)
	limit := int32(56) // int32 |  (προαιρετικό)
	tags := []string{"Inner_example"} // []string |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetFeedPosts(context.Background()).TenantId(tenantId).AfterId(afterId).Limit(limit).Tags(tags).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetFeedPosts``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απόκριση από `GetFeedPosts`: GetFeedPostsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetFeedPosts`: %v\n", resp)
}
[inline-code-end]