## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_banned_users_count_response.go)

## 示例

[inline-code-attrs-start title = 'GetCounts 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  (可选)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetCounts(context.Background()).TenantId(tenantId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "调用 `ModerationAPI.GetCounts` 时出错: %v\n", err)
		fmt.Fprintf(os.Stderr, "完整的 HTTP 响应: %v\n", r)
	}
	// 来自 `GetCounts` 的响应: GetBannedUsersCountResponse
	fmt.Fprintf(os.Stdout, "来自 `ModerationAPI.GetCounts` 的响应: %v\n", resp)
}
[inline-code-end]