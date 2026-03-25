## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | Abfrage | Ja |  |
| id | string | Pfad | Ja |  |
| userId | string | Abfrage | Nein |  |

## Antwort

Gibt zurück: [`GetTicket200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_ticket_200_response.go)

## Beispiel

[inline-code-attrs-start title = 'GetTicket Beispiel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTicket(context.Background(), id).TenantId(tenantId).UserId(userId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTicket``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// Antwort von `GetTicket`: GetTicket200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTicket`: %v\n", resp)
}
[inline-code-end]