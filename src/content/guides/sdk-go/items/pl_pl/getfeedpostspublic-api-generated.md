---
req
tenantId
afterId

## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| afterId | string | query | Nie |  |
| limit | integer | query | Nie |  |
| tags | array | query | Nie |  |
| sso | string | query | Nie |  |
| isCrawler | boolean | query | Nie |  |
| includeUserInfo | boolean | query | Nie |  |

## Odpowiedź

Zwraca: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_public_feed_posts_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład GetFeedPostsPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterId := "afterId_example" // string |  (opcjonalne)
	limit := int32(56) // int32 |  (opcjonalne)
	tags := []string{"Inner_example"} // []string |  (opcjonalne)
	sso := "sso_example" // string |  (opcjonalne)
	isCrawler := true // bool |  (opcjonalne)
	includeUserInfo := true // bool |  (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetFeedPostsPublic(context.Background(), tenantId).AfterId(afterId).Limit(limit).Tags(tags).Sso(sso).IsCrawler(isCrawler).IncludeUserInfo(includeUserInfo).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetFeedPostsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetFeedPostsPublic`: PublicFeedPostsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetFeedPostsPublic`: %v\n", resp)
}
[inline-code-end]

---