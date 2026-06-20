## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| userId | string | query | No |  |
| sso | string | query | No |  |

## 回應

回傳：[`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_trust_factor_response.go)

## 範例

[inline-code-attrs-start title = 'GetTrustFactor 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	userId := "userId_example" // string |  (選用)
	sso := "sso_example" // string |  (選用)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetTrustFactor(context.Background()).UserId(userId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetTrustFactor``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 來自 `GetTrustFactor` 的回應：GetUserTrustFactorResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetTrustFactor`: %v\n", resp)
}
[inline-code-end]