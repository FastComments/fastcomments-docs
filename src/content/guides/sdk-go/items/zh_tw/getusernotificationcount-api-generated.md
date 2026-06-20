---
## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_notification_count_response.go)

## 範例

[inline-code-attrs-start title = 'GetUserNotificationCount 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sso := "sso_example" // string |  （選填）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserNotificationCount(context.Background()).TenantId(tenantId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserNotificationCount``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetUserNotificationCount` 的回應：GetUserNotificationCountResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserNotificationCount`: %v\n", resp)
}
[inline-code-end]

---