## 參數

| 名稱 | 型別 | 位置 | 必要 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| commentId | string | path | 是 |  |
| isFlagged | boolean | query | 是 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_flag_comment_public_200_response.go)

## 範例

[inline-code-attrs-start title = 'FlagCommentPublic 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	isFlagged := true // bool | 
	sso := "sso_example" // string |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.FlagCommentPublic(context.Background(), commentId).TenantId(tenantId).IsFlagged(isFlagged).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.FlagCommentPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 來自 `FlagCommentPublic` 的回應: FlagCommentPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.FlagCommentPublic`: %v\n", resp)
}
[inline-code-end]