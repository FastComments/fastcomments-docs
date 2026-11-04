## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## 回應

回傳: [`GetTenantResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tenant_response.go)

## 範例

[inline-code-attrs-start title = 'GetTenant 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // 字串 | 
	id := "id_example" // 字串 | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTenant(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTenant``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetTenant` 的回應: GetTenantResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTenant`: %v\n", resp)
}
[inline-code-end]