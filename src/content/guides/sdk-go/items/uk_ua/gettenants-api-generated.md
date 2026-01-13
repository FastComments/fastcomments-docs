## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| meta | string | query | Ні |  |
| skip | number | query | Ні |  |

## Відповідь

Повертає: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tenants_200_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад GetTenants'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	meta := "meta_example" // string |  (необов'язково)
	skip := float64(1.2) // float64 |  (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTenants(context.Background()).TenantId(tenantId).Meta(meta).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTenants``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь від `GetTenants`: GetTenants200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTenants`: %v\n", resp)
}
[inline-code-end]