## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| userId | string | query | No |  |
| state | number | query | No |  |
| skip | number | query | No |  |
| limit | number | query | No |  |

## Respuesta

Devuelve: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tickets_200_response.go)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de GetTickets'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (opcional)
	state := float64(1.2) // float64 |  (opcional)
	skip := float64(1.2) // float64 |  (opcional)
	limit := float64(1.2) // float64 |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTickets(context.Background()).TenantId(tenantId).UserId(userId).State(state).Skip(skip).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTickets``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respuesta de `GetTickets`: GetTickets200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTickets`: %v\n", resp)
}
[inline-code-end]

---