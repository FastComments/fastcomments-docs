## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | いいえ |  |
| trustFactor | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_set_user_trust_factor_response.go)

## 例

[inline-code-attrs-start title = 'SetTrustFactor の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	userId := "userId_example" // string |  (オプション)
	trustFactor := "trustFactor_example" // string |  (オプション)
	sso := "sso_example" // string |  (オプション)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.SetTrustFactor(context.Background()).UserId(userId).TrustFactor(trustFactor).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.SetTrustFactor``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `SetTrustFactor` のレスポンス: SetUserTrustFactorResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.SetTrustFactor`: %v\n", resp)
}
[inline-code-end]