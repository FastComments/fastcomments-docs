## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| userId | string | query | Όχι |  |
| state | number | query | Όχι |  |
| skip | number | query | Όχι |  |
| limit | number | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tickets_200_response.go)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα GetTickets'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (προαιρετικό)
	state := float64(1.2) // float64 |  (προαιρετικό)
	skip := float64(1.2) // float64 |  (προαιρετικό)
	limit := float64(1.2) // float64 |  (προαιρετικό)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTickets(context.Background()).TenantId(tenantId).UserId(userId).State(state).Skip(skip).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTickets``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// απάντηση από `GetTickets`: GetTickets200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTickets`: %v\n", resp)
}
[inline-code-end]