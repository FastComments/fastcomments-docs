## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| skip | integer | query | 否 |  |

## 回應

回傳： [`GetSSOUsers200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_sso_users_200_response.go)

## 範例

[inline-code-attrs-start title = 'GetSSOUsers 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	skip := int32(56) // int32 |  (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetSSOUsers(context.Background()).TenantId(tenantId).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetSSOUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 從 `GetSSOUsers` 的回應: GetSSOUsers200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetSSOUsers`: %v\n", resp)
}
[inline-code-end]