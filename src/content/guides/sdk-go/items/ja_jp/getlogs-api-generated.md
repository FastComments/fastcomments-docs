## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| commentId | string | パス | はい |  |
| sso | string | クエリ | いいえ |  |

## レスポンス

戻り値: [`ModerationAPIGetLogsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_get_logs_response.go)

## 例

[inline-code-attrs-start title = 'GetLogsの例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (オプション)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetLogs(context.Background(), commentId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetLogs``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetLogs`のレスポンス: ModerationAPIGetLogsResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetLogs`: %v\n", resp)
}
[inline-code-end]