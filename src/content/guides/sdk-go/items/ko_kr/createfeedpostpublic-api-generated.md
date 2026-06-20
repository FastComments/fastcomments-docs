## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| broadcastId | string | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_feed_post_response.go)

## 예제

[inline-code-attrs-start title = 'CreateFeedPostPublic 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.CreateFeedPostPublic(context.Background(), tenantId).CreateFeedPostParams(createFeedPostParams).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.CreateFeedPostPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `CreateFeedPostPublic`의 응답: CreateFeedPostResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.CreateFeedPostPublic`: %v\n", resp)
}
[inline-code-end]