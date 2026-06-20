## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| commentId | string | path | はい |  |
| includeEmail | boolean | query | いいえ |  |
| includeIP | boolean | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_comment_response.go)

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
	commentId := "commentId_example" // string | 
	includeEmail := true // bool |  （任意）
	includeIP := true // bool |  （任意）
	sso := "sso_example" // string |  （任意）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetModerationComment(context.Background(), commentId).IncludeEmail(includeEmail).IncludeIP(includeIP).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetModerationComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetModerationComment` のレスポンス: ModerationAPICommentResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetModerationComment`: %v\n", resp)
}
[inline-code-end]