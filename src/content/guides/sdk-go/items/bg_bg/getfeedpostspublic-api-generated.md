req
tenantId
afterId

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | път | Да |  |
| afterId | string | заявка | Не |  |
| limit | integer | заявка | Не |  |
| tags | array | заявка | Не |  |
| sso | string | заявка | Не |  |
| isCrawler | boolean | заявка | Не |  |
| includeUserInfo | boolean | заявка | Не |  |

## Отговор

Връща: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_feed_posts_public_200_response.go)

## Пример

[inline-code-attrs-start title = 'Пример за GetFeedPostsPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterId := "afterId_example" // string |  (по избор)
	limit := int32(56) // int32 |  (по избор)
	tags := []string{"Inner_example"} // []string |  (по избор)
	sso := "sso_example" // string |  (по избор)
	isCrawler := true // bool |  (по избор)
	includeUserInfo := true // bool |  (по избор)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetFeedPostsPublic(context.Background(), tenantId).AfterId(afterId).Limit(limit).Tags(tags).Sso(sso).IsCrawler(isCrawler).IncludeUserInfo(includeUserInfo).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetFeedPostsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// отговор от `GetFeedPostsPublic`: GetFeedPostsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetFeedPostsPublic`: %v\n", resp)
}
[inline-code-end]