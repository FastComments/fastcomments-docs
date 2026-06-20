## 參數

| 名稱 | 型別 | 位置 | 必要 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 查詢 | 是 |  |
| skip | integer | 查詢 | 否 |  |

## 回應

回傳: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_sso_users_response.go)

## 範例

[inline-code-attrs-start title = 'GetSSOUsers 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	skip := int32(56) // int32 |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetSSOUsers(context.Background()).TenantId(tenantId).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetSSOUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 從 `GetSSOUsers` 回傳的回應: GetSSOUsersResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetSSOUsers`: %v\n", resp)
}
[inline-code-end]

---