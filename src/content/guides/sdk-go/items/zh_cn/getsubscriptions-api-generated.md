## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 查询 | 是 |  |
| userId | string | 查询 | 否 |  |

## 响应

返回: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_subscriptions_api_response.go)

## 示例

[inline-code-attrs-start title = 'GetSubscriptions 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (可选)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetSubscriptions(context.Background()).TenantId(tenantId).UserId(userId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetSubscriptions``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `GetSubscriptions` 的响应: GetSubscriptionsAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetSubscriptions`: %v\n", resp)
}
[inline-code-end]