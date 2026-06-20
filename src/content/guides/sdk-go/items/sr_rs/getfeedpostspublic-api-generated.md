req
tenantId
afterId

## Параметри

| Име | Тип | Локација | Потребно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| afterId | string | query | Не |  |
| limit | integer | query | Не |  |
| tags | array | query | Не |  |
| sso | string | query | Не |  |
| isCrawler | boolean | query | Не |  |
| includeUserInfo | boolean | query | Не |  |

## Одговор

Враћа: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_public_feed_posts_response.go)

## Пример

[inline-code-attrs-start title = 'GetFeedPostsPublic Example'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterId := "afterId_example" // string |  (опционо)
	limit := int32(56) // int32 |  (опционо)
	tags := []string{"Inner_example"} // []string |  (опционо)
	sso := "sso_example" // string |  (опционо)
	isCrawler := true // bool |  (опционо)
	includeUserInfo := true // bool |  (опционо)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetFeedPostsPublic(context.Background(), tenantId).AfterId(afterId).Limit(limit).Tags(tags).Sso(sso).IsCrawler(isCrawler).IncludeUserInfo(includeUserInfo).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetFeedPostsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// одговор из `GetFeedPostsPublic`: PublicFeedPostsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetFeedPostsPublic`: %v\n", resp)
}
[inline-code-end]