## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | sorgu | Evet |  |
| id | string | yol | Evet |  |

## Yanıt

Döndürür: [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tenant_user_response.go)

## Örnek

[inline-code-attrs-start title = 'GetTenantUser Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	resp, r, err := apiClient.DefaultAPI.GetTenantUser(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTenantUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetTenantUser` yanıtı: GetTenantUserResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTenantUser`: %v\n", resp)
}
[inline-code-end]

---