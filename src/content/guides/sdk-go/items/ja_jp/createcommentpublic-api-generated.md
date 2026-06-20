## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい |  |
| broadcastId | string | query | はい |  |
| sessionId | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

返却値: [`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_save_comments_response_with_presence.go)

## 例

[inline-code-attrs-start title = 'CreateCommentPublic の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sessionId := "sessionId_example" // string |  (任意)
	sso := "sso_example" // string |  (任意)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.CreateCommentPublic(context.Background(), tenantId).UrlId(urlId).BroadcastId(broadcastId).CommentData(commentData).SessionId(sessionId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.CreateCommentPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `CreateCommentPublic` のレスポンス: SaveCommentsResponseWithPresence
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.CreateCommentPublic`: %v\n", resp)
}
[inline-code-end]