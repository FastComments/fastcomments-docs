---
## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |

## Respuesta

Devuelve: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_moderator_response.go)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de CreateModerator'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createModeratorBody := *openapiclient.NewCreateModeratorBody("Name_example", "Email_example") // CreateModeratorBody | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateModerator(context.Background()).TenantId(tenantId).CreateModeratorBody(createModeratorBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateModerator``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respuesta de `CreateModerator`: CreateModeratorResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateModerator`: %v\n", resp)
}
[inline-code-end]

---