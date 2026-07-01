## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回：[`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_trust_factor_response.go)

## 示例

[inline-code-attrs-start title = 'GetTrustFactor 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	userId := "userId_example" // string |  （可选）
	sso := "sso_example" // string |  （可选）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetTrustFactor(context.Background()).TenantId(tenantId).UserId(userId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "调用 `ModerationAPI.GetTrustFactor` 时出错: %v\n", err)
		fmt.Fprintf(os.Stderr, "完整 HTTP 响应: %v\n", r)
	}
	// `GetTrustFactor` 的响应：GetUserTrustFactorResponse
	fmt.Fprintf(os.Stdout, "来自 `ModerationAPI.GetTrustFactor` 的响应: %v\n", resp)
}
[inline-code-end]