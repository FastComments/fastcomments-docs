## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| skip | number | query | Nein |  |

## Antwort

Gibt zurück: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tenant_users_response.go)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für GetTenantUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	skip := float64(1.2) // float64 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTenantUsers(context.Background()).TenantId(tenantId).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTenantUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `GetTenantUsers`: GetTenantUsersResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTenantUsers`: %v\n", resp)
}
[inline-code-end]