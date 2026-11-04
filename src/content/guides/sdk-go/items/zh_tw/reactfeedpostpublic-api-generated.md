## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| postId | string | path | 是 |  |
| isUndo | boolean | query | 否 |  |
| broadcastId | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_react_feed_post_response.go)

## 範例

[inline-code-attrs-start title = 'ReactFeedPostPublic 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	isUndo := true // bool |  (可選)
	broadcastId := "broadcastId_example" // string |  (可選)
	sso := "sso_example" // string |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.ReactFeedPostPublic(context.Background(), tenantId, postId).ReactBodyParams(reactBodyParams).IsUndo(isUndo).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.ReactFeedPostPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ReactFeedPostPublic`: ReactFeedPostResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.ReactFeedPostPublic`: %v\n", resp)
}
[inline-code-end]