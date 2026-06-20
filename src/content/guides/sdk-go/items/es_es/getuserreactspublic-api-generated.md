## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| postIds | array | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_user_reacts_response.go)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de GetUserReactsPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	postIds := []string{"Inner_example"} // []string |  (opcional)
	sso := "sso_example" // string |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetUserReactsPublic(context.Background(), tenantId).PostIds(postIds).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetUserReactsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respuesta de `GetUserReactsPublic`: UserReactsResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetUserReactsPublic`: %v\n", resp)
}
[inline-code-end]