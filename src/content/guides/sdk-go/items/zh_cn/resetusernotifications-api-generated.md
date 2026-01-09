## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| afterId | string | query | 否 |  |
| afterCreatedAt | integer | query | 否 |  |
| unreadOnly | boolean | query | 否 |  |
| dmOnly | boolean | query | 否 |  |
| noDm | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_reset_user_notifications_200_response.go)

## 示例

[inline-code-attrs-start title = 'ResetUserNotifications 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	afterId := "afterId_example" // string |  (可选)
	afterCreatedAt := int64(789) // int64 |  (可选)
	unreadOnly := true // bool |  (可选)
	dmOnly := true // bool |  (可选)
	noDm := true // bool |  (可选)
	sso := "sso_example" // string |  (可选)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.ResetUserNotifications(context.Background()).TenantId(tenantId).AfterId(afterId).AfterCreatedAt(afterCreatedAt).UnreadOnly(unreadOnly).DmOnly(dmOnly).NoDm(noDm).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.ResetUserNotifications``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ResetUserNotifications`: ResetUserNotifications200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.ResetUserNotifications`: %v\n", resp)
}
[inline-code-end]