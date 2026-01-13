## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| meta | string | query | 否 |  |
| skip | number | query | 否 |  |

## 响应

返回: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tenants_200_response.go)

## 示例

[inline-code-attrs-start title = 'GetTenants 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	meta := "meta_example" // string |  (可选)
	skip := float64(1.2) // float64 |  (可选)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTenants(context.Background()).TenantId(tenantId).Meta(meta).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTenants``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `GetTenants` 的响应: GetTenants200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTenants`: %v\n", resp)
}
[inline-code-end]