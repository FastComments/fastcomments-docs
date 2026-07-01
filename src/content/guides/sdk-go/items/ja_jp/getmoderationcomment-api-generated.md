## パラメータ

| 名前 | タイプ | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| commentId | string | path | はい |  |
| includeEmail | boolean | query | いいえ |  |
| includeIP | boolean | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

返却: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_comment_response.go)

## 例

[inline-code-attrs-start title = 'GetModerationComment の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string | 
	includeEmail := true // bool |  (optional) (オプション)
	includeIP := true // bool |  (optional) (オプション)
	sso := "sso_example" // string |  (optional) (オプション)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetModerationComment(context.Background(), commentId).TenantId(tenantId).IncludeEmail(includeEmail).IncludeIP(includeIP).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetModerationComment``: %v\n", err) // `ModerationAPI.GetModerationComment` の呼び出し時のエラー
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r) // 完全な HTTP 応答
	}
	// `GetModerationComment` からのレスポンス: ModerationAPICommentResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetModerationComment`: %v\n", resp)
}
[inline-code-end]