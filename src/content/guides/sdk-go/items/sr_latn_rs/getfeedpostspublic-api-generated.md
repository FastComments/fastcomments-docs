req
tenantId
afterId

## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| afterId | string | query | Ne |  |
| limit | integer | query | Ne |  |
| tags | array | query | Ne |  |
| sso | string | query | Ne |  |
| isCrawler | boolean | query | Ne |  |
| includeUserInfo | boolean | query | Ne |  |

## Odgovor

Vraća: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_feed_posts_public_200_response.go)

## Primer

[inline-code-attrs-start title = 'Primer GetFeedPostsPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterId := "afterId_example" // string |  (neobavezno)
	limit := int32(56) // int32 |  (neobavezno)
	tags := []string{"Inner_example"} // []string |  (neobavezno)
	sso := "sso_example" // string |  (neobavezno)
	isCrawler := true // bool |  (neobavezno)
	includeUserInfo := true // bool |  (neobavezno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetFeedPostsPublic(context.Background(), tenantId).AfterId(afterId).Limit(limit).Tags(tags).Sso(sso).IsCrawler(isCrawler).IncludeUserInfo(includeUserInfo).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetFeedPostsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor od `GetFeedPostsPublic`: GetFeedPostsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetFeedPostsPublic`: %v\n", resp)
}
[inline-code-end]