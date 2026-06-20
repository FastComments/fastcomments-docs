## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| commentId | string | 路徑 | 是 |  |
| includeByUserIdAndEmail | boolean | 查詢 | 否 |  |
| includeByIP | boolean | 查詢 | 否 |  |
| includeByEmailDomain | boolean | 查詢 | 否 |  |
| sso | string | 查詢 | 否 |  |

## 回應

回傳：[`PreBanSummary`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_pre_ban_summary.go)

## 範例

[inline-code-attrs-start title = 'GetPreBanSummary 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	includeByUserIdAndEmail := true // bool |  (可選)
	includeByIP := true // bool |  (可選)
	includeByEmailDomain := true // bool |  (可選)
	sso := "sso_example" // string |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetPreBanSummary(context.Background(), commentId).IncludeByUserIdAndEmail(includeByUserIdAndEmail).IncludeByIP(includeByIP).IncludeByEmailDomain(includeByEmailDomain).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetPreBanSummary``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 從 `GetPreBanSummary` 的回應: PreBanSummary
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetPreBanSummary`: %v\n", resp)
}
[inline-code-end]