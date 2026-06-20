## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | はい |  |
| includeByUserIdAndEmail | boolean | query | いいえ |  |
| includeByIP | boolean | query | いいえ |  |
| includeByEmailDomain | boolean | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`PreBanSummary`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_pre_ban_summary.go)

## 例

[inline-code-attrs-start title = 'GetPreBanSummary の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	includeByUserIdAndEmail := true // bool |  (任意)
	includeByIP := true // bool |  (任意)
	includeByEmailDomain := true // bool |  (任意)
	sso := "sso_example" // string |  (任意)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetPreBanSummary(context.Background(), commentId).IncludeByUserIdAndEmail(includeByUserIdAndEmail).IncludeByIP(includeByIP).IncludeByEmailDomain(includeByEmailDomain).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetPreBanSummary``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetPreBanSummary` からのレスポンス: PreBanSummary
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetPreBanSummary`: %v\n", resp)
}
[inline-code-end]

---