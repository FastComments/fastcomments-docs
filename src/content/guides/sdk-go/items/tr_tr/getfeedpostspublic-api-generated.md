req
tenantId
afterId

## Parametreler

| İsim | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| afterId | string | query | Hayır |  |
| limit | integer | query | Hayır |  |
| tags | array | query | Hayır |  |
| sso | string | query | Hayır |  |
| isCrawler | boolean | query | Hayır |  |
| includeUserInfo | boolean | query | Hayır |  |

## Yanıt

Döndürür: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_public_feed_posts_response.go)

## Örnek

[inline-code-attrs-start title = 'GetFeedPostsPublic Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterId := "afterId_example" // string |  (isteğe bağlı)
	limit := int32(56) // int32 |  (isteğe bağlı)
	tags := []string{"Inner_example"} // []string |  (isteğe bağlı)
	sso := "sso_example" // string |  (isteğe bağlı)
	isCrawler := true // bool |  (isteğe bağlı)
	includeUserInfo := true // bool |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetFeedPostsPublic(context.Background(), tenantId).AfterId(afterId).Limit(limit).Tags(tags).Sso(sso).IsCrawler(isCrawler).IncludeUserInfo(includeUserInfo).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetFeedPostsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetFeedPostsPublic`'ten cevap: PublicFeedPostsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetFeedPostsPublic`: %v\n", resp)
}
[inline-code-end]