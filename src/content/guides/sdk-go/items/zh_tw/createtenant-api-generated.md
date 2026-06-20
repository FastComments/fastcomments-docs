## 參數

| 名稱 | 型別 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |

## 回應

回傳: [`CreateTenantResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_tenant_response.go)

## 範例

[inline-code-attrs-start title = 'CreateTenant 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
    "time"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	createTenantBody := *openapiclient.NewCreateTenantBody("Name_example", []openapiclient.APIDomainConfiguration{*openapiclient.NewAPIDomainConfiguration("Id_example", "Domain_example", time.Now())}) // CreateTenantBody | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateTenant(context.Background()).TenantId(tenantId).CreateTenantBody(createTenantBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateTenant``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateTenant`: CreateTenantResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateTenant`: %v\n", resp)
}
[inline-code-end]