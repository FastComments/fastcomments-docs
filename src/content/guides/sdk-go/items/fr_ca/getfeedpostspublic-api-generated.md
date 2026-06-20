req
tenantId
afterId

## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| afterId | string | query | Non |  |
| limit | integer | query | Non |  |
| tags | array | query | Non |  |
| sso | string | query | Non |  |
| isCrawler | boolean | query | Non |  |
| includeUserInfo | boolean | query | Non |  |

## Réponse

Renvoie: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_public_feed_posts_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple GetFeedPostsPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterId := "afterId_example" // string |  (facultatif)
	limit := int32(56) // int32 |  (facultatif)
	tags := []string{"Inner_example"} // []string |  (facultatif)
	sso := "sso_example" // string |  (facultatif)
	isCrawler := true // bool |  (facultatif)
	includeUserInfo := true // bool |  (facultatif)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetFeedPostsPublic(context.Background(), tenantId).AfterId(afterId).Limit(limit).Tags(tags).Sso(sso).IsCrawler(isCrawler).IncludeUserInfo(includeUserInfo).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetFeedPostsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `GetFeedPostsPublic`: PublicFeedPostsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetFeedPostsPublic`: %v\n", resp)
}
[inline-code-end]