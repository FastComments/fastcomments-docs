## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlIdWS | string | query | Yes |  |
| userIds | string | query | Yes |  |

## 响应

返回：[`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_presence_statuses_response.go)

## 示例

[inline-code-attrs-start title = 'GetUserPresenceStatuses 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // 字符串 | 
	urlIdWS := "urlIdWS_example" // 字符串 | 
	userIds := "userIds_example" // 字符串 | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserPresenceStatuses(context.Background()).TenantId(tenantId).UrlIdWS(urlIdWS).UserIds(userIds).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserPresenceStatuses``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetUserPresenceStatuses`: GetUserPresenceStatusesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserPresenceStatuses`: %v\n", resp)
}
[inline-code-end]