## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| userId | string | query | Sí |  |

## Respuesta

Devuelve: [`CreateTicketResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_ticket_response.go)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de CreateTicket'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string | 
	createTicketBody := *openapiclient.NewCreateTicketBody("Subject_example") // CreateTicketBody | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateTicket(context.Background()).TenantId(tenantId).UserId(userId).CreateTicketBody(createTicketBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateTicket``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respuesta de `CreateTicket`: CreateTicketResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateTicket`: %v\n", resp)
}
[inline-code-end]