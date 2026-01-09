## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | パス | はい |  |
| commentId | string | パス | はい |  |
| broadcastId | string | クエリ | はい |  |
| sso | string | クエリ | いいえ |  |

## レスポンス

返却: [`LockComment200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_lock_comment_200_response.go)

## 例

[inline-code-attrs-start title = 'LockComment の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	resp, r, err := apiClient.PublicAPI.LockComment(context.Background(), tenantId, commentId).BroadcastId(broadcastId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.LockComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `LockComment`からのレスポンス: LockComment200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.LockComment`: %v\n", resp)
}
[inline-code-end]

---