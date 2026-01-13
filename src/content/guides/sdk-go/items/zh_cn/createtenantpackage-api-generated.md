## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |

## 响应

返回: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_tenant_package_200_response.go)

## 示例

[inline-code-attrs-start title = 'CreateTenantPackage 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createTenantPackageBody := *openapiclient.NewCreateTenantPackageBody("Name_example", float64(123), float64(123), float64(123), float64(123), float64(123), float64(123), float64(123), float64(123), false, "ForWhoText_example", []string{"FeatureTaglines_example"}, false) // CreateTenantPackageBody | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateTenantPackage(context.Background()).TenantId(tenantId).CreateTenantPackageBody(createTenantPackageBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateTenantPackage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `CreateTenantPackage` 的响应: CreateTenantPackage200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateTenantPackage`: %v\n", resp)
}
[inline-code-end]