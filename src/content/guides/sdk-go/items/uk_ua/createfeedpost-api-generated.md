## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| broadcastId | string | query | Ні |  |
| isLive | boolean | query | Ні |  |
| doSpamCheck | boolean | query | Ні |  |
| skipDupCheck | boolean | query | Ні |  |

## Відповідь

Повертає: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_feed_posts_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад CreateFeedPost'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createFeedPostParams := *openapiclient.NewCreateFeedPostParams() // CreateFeedPostParams | 
	broadcastId := "broadcastId_example" // string |  (необов'язково)
	isLive := true // bool |  (необов'язково)
	doSpamCheck := true // bool |  (необов'язково)
	skipDupCheck := true // bool |  (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateFeedPost(context.Background()).TenantId(tenantId).CreateFeedPostParams(createFeedPostParams).BroadcastId(broadcastId).IsLive(isLive).DoSpamCheck(doSpamCheck).SkipDupCheck(skipDupCheck).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateFeedPost``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateFeedPost`: CreateFeedPostsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateFeedPost`: %v\n", resp)
}
[inline-code-end]