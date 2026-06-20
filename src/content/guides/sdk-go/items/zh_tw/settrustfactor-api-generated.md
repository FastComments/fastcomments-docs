## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| userId | string | 查詢 | 否 |  |
| trustFactor | string | 查詢 | 否 |  |
| sso | string | 查詢 | 否 |  |

## 回應

回傳: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_set_user_trust_factor_response.go)

## 範例

[inline-code-attrs-start title = 'SetTrustFactor 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	userId := "userId_example" // string |  (可選)
	trustFactor := "trustFactor_example" // string |  (可選)
	sso := "sso_example" // string |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.SetTrustFactor(context.Background()).UserId(userId).TrustFactor(trustFactor).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.SetTrustFactor``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 來自 `SetTrustFactor` 的回應: SetUserTrustFactorResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.SetTrustFactor`: %v\n", resp)
}
[inline-code-end]