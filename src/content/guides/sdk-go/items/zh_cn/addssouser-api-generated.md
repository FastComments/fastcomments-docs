## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |

## Response

返回: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_add_sso_user_api_response.go)

## 示例

[inline-code-attrs-start title = 'AddSSOUser 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 租户 ID
	createAPISSOUserData := *openapiclient.NewCreateAPISSOUserData("Email_example", "Username_example", "Id_example") // CreateAPISSOUserData | 创建 AP SSO 用户数据

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.AddSSOUser(context.Background()).TenantId(tenantId).CreateAPISSOUserData(createAPISSOUserData).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.AddSSOUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `AddSSOUser` 的响应：AddSSOUserAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.AddSSOUser`: %v\n", resp)
}
[inline-code-end]

---