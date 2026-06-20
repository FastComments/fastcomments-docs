## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 路徑 | 是 |  |
| broadcastId | string | 查詢 | 否 |  |
| sso | string | 查詢 | 否 |  |

## 回應

回傳: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_feed_post_response.go)

## 範例

[inline-code-attrs-start title = 'CreateFeedPostPublic 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	broadcastId := "broadcastId_example" // string |  (可選)
	sso := "sso_example" // string |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.CreateFeedPostPublic(context.Background(), tenantId).CreateFeedPostParams(createFeedPostParams).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.CreateFeedPostPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `CreateFeedPostPublic` 的回應：CreateFeedPostResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.CreateFeedPostPublic`: %v\n", resp)
}
[inline-code-end]