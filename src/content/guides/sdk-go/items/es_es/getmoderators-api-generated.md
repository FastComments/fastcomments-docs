## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| skip | number | query | No |  |

## Respuesta

Devuelve: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_moderators_response.go)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de GetModerators'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	skip := float64(1.2) // float64 |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetModerators(context.Background()).TenantId(tenantId).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetModerators``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetModerators`: GetModeratorsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetModerators`: %v\n", resp)
}
[inline-code-end]