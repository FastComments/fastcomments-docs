## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| userId | string | query | Ne |  |
| state | number | query | Ne |  |
| skip | number | query | Ne |  |
| limit | number | query | Ne |  |

## Odgovor

Vraća: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tickets_200_response.go)

## Primer

[inline-code-attrs-start title = 'GetTickets Primer'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (neobavezno)
	state := float64(1.2) // float64 |  (neobavezno)
	skip := float64(1.2) // float64 |  (neobavezno)
	limit := float64(1.2) // float64 |  (neobavezno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTickets(context.Background()).TenantId(tenantId).UserId(userId).State(state).Skip(skip).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTickets``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor od `GetTickets`: GetTickets200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTickets`: %v\n", resp)
}
[inline-code-end]