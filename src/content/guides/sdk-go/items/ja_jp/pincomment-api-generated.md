## パラメーター

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| commentId | string | path | はい |  |
| broadcastId | string | query | はい |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`PinComment200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_pin_comment_200_response.go)

## 例

[inline-code-attrs-start title = 'PinComment の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string | 
	broadcastId := "broadcastId_example" // string | 
	sso := "sso_example" // string |  (オプション)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.PinComment(context.Background(), tenantId, commentId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.PinComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `PinComment` からのレスポンス: PinComment200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.PinComment`: %v\n", resp)
}
[inline-code-end]