## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| broadcastId | string | query | 아니오 |  |
| isLive | boolean | query | 아니오 |  |
| doSpamCheck | boolean | query | 아니오 |  |
| skipDupCheck | boolean | query | 아니오 |  |

## 응답

반환: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_feed_posts_response.go)

## 예제

[inline-code-attrs-start title = 'CreateFeedPost Example'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	broadcastId := "broadcastId_example" // string |  (선택 사항)
	isLive := true // bool |  (선택 사항)
	doSpamCheck := true // bool |  (선택 사항)
	skipDupCheck := true // bool |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateFeedPost(context.Background()).TenantId(tenantId).CreateFeedPostParams(createFeedPostParams).BroadcastId(broadcastId).IsLive(isLive).DoSpamCheck(doSpamCheck).SkipDupCheck(skipDupCheck).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateFeedPost``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `CreateFeedPost`의 응답: CreateFeedPostsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateFeedPost`: %v\n", resp)
}
[inline-code-end]