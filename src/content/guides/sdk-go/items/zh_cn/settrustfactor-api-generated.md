## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| userId | string | query | 否 |  |
| trustFactor | string | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回：[`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_set_user_trust_factor_response.go)

## 示例

[inline-code-attrs-start title = 'SetTrustFactor 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	userId := "userId_example" // string |  （可选）
	trustFactor := "trustFactor_example" // string |  （可选）
	sso := "sso_example" // string |  （可选）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.SetTrustFactor(context.Background()).UserId(userId).TrustFactor(trustFactor).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.SetTrustFactor``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `SetTrustFactor` 的响应: SetUserTrustFactorResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.SetTrustFactor`: %v\n", resp)
}
[inline-code-end]

---