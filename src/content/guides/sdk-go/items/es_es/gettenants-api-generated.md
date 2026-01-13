## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| meta | string | query | No |  |
| skip | number | query | No |  |

## Respuesta

Devuelve: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tenants_200_response.go)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de GetTenants'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	meta := "meta_example" // string |  (opcional)
	skip := float64(1.2) // float64 |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTenants(context.Background()).TenantId(tenantId).Meta(meta).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTenants``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respuesta de `GetTenants`: GetTenants200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTenants`: %v\n", resp)
}
[inline-code-end]