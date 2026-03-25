---
## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| userId | string | query | Tak |  |
| id | string | path | Tak |  |

## Odpowiedź

Zwraca: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_change_ticket_state_200_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład ChangeTicketState'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 
	changeTicketStateBody := *openapiclient.NewChangeTicketStateBody(int32(123)) // ChangeTicketStateBody | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.ChangeTicketState(context.Background(), id).TenantId(tenantId).UserId(userId).ChangeTicketStateBody(changeTicketStateBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.ChangeTicketState``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `ChangeTicketState`: ChangeTicketState200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.ChangeTicketState`: %v\n", resp)
}
[inline-code-end]

---