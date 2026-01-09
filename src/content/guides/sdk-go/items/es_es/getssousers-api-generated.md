---
## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| skip | integer | query | No |  |

## Respuesta

Devuelve: [`GetSSOUsers200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_sso_users_200_response.go)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de GetSSOUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	skip := int32(56) // int32 |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetSSOUsers(context.Background()).TenantId(tenantId).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetSSOUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respuesta de `GetSSOUsers`: GetSSOUsers200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetSSOUsers`: %v\n", resp)
}
[inline-code-end]

---