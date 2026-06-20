## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 路徑 | 是 |  |
| urlId | string | 查詢 | 是 |  |
| broadcastId | string | 查詢 | 是 |  |
| sessionId | string | 查詢 | 否 |  |
| sso | string | 查詢 | 否 |  |

## 回應

傳回: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_save_comments_response_with_presence.go)

## 範例

[inline-code-attrs-start title = 'CreateCommentPublic 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 
	broadcastId := "broadcastId_example" // string | 
	commentData := *openapiclient.NewCommentData("CommenterName_example", "Comment_example", "Url_example", "UrlId_example") // CommentData | 
	sessionId := "sessionId_example" // string |  (可選)
	sso := "sso_example" // string |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.CreateCommentPublic(context.Background(), tenantId).UrlId(urlId).BroadcastId(broadcastId).CommentData(commentData).SessionId(sessionId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.CreateCommentPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `CreateCommentPublic` 的回應：SaveCommentsResponseWithPresence
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.CreateCommentPublic`: %v\n", resp)
}
[inline-code-end]