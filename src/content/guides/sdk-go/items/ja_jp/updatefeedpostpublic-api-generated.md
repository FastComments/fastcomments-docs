## パラメータ

| Name | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| postId | string | path | はい |  |
| broadcastId | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

返却値: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_feed_post_response.go)

## 例

[inline-code-attrs-start title = 'UpdateFeedPostPublic の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	updateFeedPostParams := *openapiclient.NewUpdateFeedPostParams() // UpdateFeedPostParams | 
	broadcastId := "broadcastId_example" // string |  (オプション)
	sso := "sso_example" // string |  (オプション)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UpdateFeedPostPublic(context.Background(), tenantId, postId).UpdateFeedPostParams(updateFeedPostParams).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UpdateFeedPostPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `UpdateFeedPostPublic` のレスポンス: CreateFeedPostResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UpdateFeedPostPublic`: %v\n", resp)
}
[inline-code-end]