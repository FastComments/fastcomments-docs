---
当前在线的页面查看者：其 websocket 会话当前订阅了该页面的人。返回 anonCount + totalCount（房间范围的订阅者，包括我们不枚举的匿名观众）。

## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 | Page URL identifier (cleaned server-side). |
| afterName | string | query | 否 | 游标：传入来自上一个响应的 nextAfterName。 |
| afterUserId | string | query | 否 | 游标决胜项：传入来自上一个响应的 nextAfterUserId。在设置 afterName 时为必要，以防止同名导致条目丢失。 |

## 响应

返回: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_online_response.go)

## 示例

[inline-code-attrs-start title = 'GetOnlineUsers 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 页面 URL 标识符（服务器端已清理）。
	afterName := "afterName_example" // string | 游标：传入来自上一个响应的 nextAfterName。 (optional)
	afterUserId := "afterUserId_example" // string | 游标决胜项：传入来自上一个响应的 nextAfterUserId。Required when afterName is set so name-ties don't drop entries. (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOnlineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOnlineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetOnlineUsers`: PageUsersOnlineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOnlineUsers`: %v\n", resp)
}
[inline-code-end]

---