---
此页面的过去评论者（当前不在线）。按 displayName 排序。
在用尽 /users/online 后，使用此接口来呈现 “成员” 部分。
基于 commenterName 的游标分页：服务器沿部分索引 {tenantId, urlId, commenterName} 从 afterName 向前通过 $gt 遍历，无 $skip 成本。

## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 页面 URL 标识符（服务器端已清理）。 |
| afterName | string | query | No | 游标：传入上一次响应中的 nextAfterName。 |
| afterUserId | string | query | No | 游标决胜项：传入上一次响应中的 nextAfterUserId。当设置 afterName 时需要此项，以免同名时丢失条目。 |

## 响应

返回：[`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

## 示例

[inline-code-attrs-start title = 'GetOfflineUsers 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 租户 ID
	urlId := "urlId_example" // string | 页面 URL 标识符（服务器端已清理）。
	afterName := "afterName_example" // string | 游标：传入上一次响应中的 nextAfterName。（可选）
	afterUserId := "afterUserId_example" // string | 游标决胜项：传入上一次响应中的 nextAfterUserId。当设置 afterName 时需要此项，以免同名时丢失条目。（可选）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `GetOfflineUsers` 的响应：PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]

---