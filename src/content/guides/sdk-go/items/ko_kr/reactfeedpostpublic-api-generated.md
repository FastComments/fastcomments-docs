## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| postId | string | path | 예 |  |
| isUndo | boolean | query | 아니요 |  |
| broadcastId | string | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_react_feed_post_response.go)

## 예제

[inline-code-attrs-start title = 'ReactFeedPostPublic 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	postId := "postId_example" // string | 
	reactBodyParams := *openapiclient.NewReactBodyParams() // ReactBodyParams | 
	isUndo := true // bool |  (선택 사항)
	broadcastId := "broadcastId_example" // string |  (선택 사항)
	sso := "sso_example" // string |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.ReactFeedPostPublic(context.Background(), tenantId, postId).ReactBodyParams(reactBodyParams).IsUndo(isUndo).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.ReactFeedPostPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `ReactFeedPostPublic`의 응답: ReactFeedPostResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.ReactFeedPostPublic`: %v\n", resp)
}
[inline-code-end]