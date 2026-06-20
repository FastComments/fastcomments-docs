## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| sso | string | 查詢 | 否 |  |

## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_empty_response.go)

## 範例

[inline-code-attrs-start title = 'PostBanUserUndo 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	banUserUndoParams := *openapiclient.NewBanUserUndoParams(*openapiclient.NewAPIBanUserChangeLog()) // BanUserUndoParams | 
	sso := "sso_example" // string |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostBanUserUndo(context.Background()).BanUserUndoParams(banUserUndoParams).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.PostBanUserUndo``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 從 `PostBanUserUndo` 的回應：APIEmptyResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.PostBanUserUndo`: %v\n", resp)
}
[inline-code-end]