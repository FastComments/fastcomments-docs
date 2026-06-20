## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| skip | number | query | No |  |

## Ответ

Возвращает: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tenant_packages_response.go)

## Пример

[inline-code-attrs-start title = 'Пример GetTenantPackages'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	skip := float64(1.2) // float64 |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTenantPackages(context.Background()).TenantId(tenantId).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTenantPackages``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `GetTenantPackages`: GetTenantPackagesResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTenantPackages`: %v\n", resp)
}
[inline-code-end]