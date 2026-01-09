## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| email | string | path | 是 |  |

## 响应

返回: [`GetSSOUserByEmailAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_sso_user_by_email_api_response.go)

## 示例

[inline-code-attrs-start title = 'GetSSOUserByEmail 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	email := "email_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetSSOUserByEmail(context.Background(), email).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetSSOUserByEmail``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `GetSSOUserByEmail` 的响应: GetSSOUserByEmailAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetSSOUserByEmail`: %v\n", resp)
}
[inline-code-end]