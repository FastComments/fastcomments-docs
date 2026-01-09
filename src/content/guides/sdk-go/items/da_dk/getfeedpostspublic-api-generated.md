req
tenantId
afterId

## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| afterId | string | query | Nej |  |
| limit | integer | query | Nej |  |
| tags | array | query | Nej |  |
| sso | string | query | Nej |  |
| isCrawler | boolean | query | Nej |  |
| includeUserInfo | boolean | query | Nej |  |

## Respons

Returnerer: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_feed_posts_public_200_response.go)

## Eksempel

[inline-code-attrs-start title = 'GetFeedPostsPublic Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterId := "afterId_example" // string |  (valgfri)
	limit := int32(56) // int32 |  (valgfri)
	tags := []string{"Inner_example"} // []string |  (valgfri)
	sso := "sso_example" // string |  (valgfri)
	isCrawler := true // bool |  (valgfri)
	includeUserInfo := true // bool |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetFeedPostsPublic(context.Background(), tenantId).AfterId(afterId).Limit(limit).Tags(tags).Sso(sso).IsCrawler(isCrawler).IncludeUserInfo(includeUserInfo).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetFeedPostsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `GetFeedPostsPublic`: GetFeedPostsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetFeedPostsPublic`: %v\n", resp)
}
[inline-code-end]