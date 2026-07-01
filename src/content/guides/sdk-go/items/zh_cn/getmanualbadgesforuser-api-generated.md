## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| badgesUserId | string | query | No |  |
| commentId | string | query | No |  |
| sso | string | query | No |  |

## 响应

返回：[`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_manual_badges_response.go)

## 示例

[inline-code-attrs-start title = 'GetManualBadgesForUser 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	badgesUserId := "badgesUserId_example" // string |  （可选）
	commentId := "commentId_example" // string |  （可选）
	sso := "sso_example" // string |  （可选）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetManualBadgesForUser(context.Background()).TenantId(tenantId).BadgesUserId(badgesUserId).CommentId(commentId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "调用 `ModerationAPI.GetManualBadgesForUser`` 时出错：%v\n", err)
		fmt.Fprintf(os.Stderr, "完整的 HTTP 响应：%v\n", r)
	}
	// 来自 `GetManualBadgesForUser`：GetUserManualBadgesResponse
	fmt.Fprintf(os.Stdout, "来自 `ModerationAPI.GetManualBadgesForUser` 的响应：%v\n", resp)
}
[inline-code-end]

---