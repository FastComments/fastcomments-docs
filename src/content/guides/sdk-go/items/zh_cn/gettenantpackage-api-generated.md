## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 响应

返回: [`GetTenantPackage200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tenant_package_200_response.go)

## 示例

[inline-code-attrs-start title = 'GetTenantPackage 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTenantPackage(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTenantPackage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `GetTenantPackage` 的响应: GetTenantPackage200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTenantPackage`: %v\n", resp)
}
[inline-code-end]