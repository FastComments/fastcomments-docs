## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| state | number | query | No |  |
| skip | number | query | No |  |
| limit | number | query | No |  |

## Yanıt

Döndürür: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tickets_200_response.go)

## Örnek

[inline-code-attrs-start title = 'GetTickets Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (isteğe bağlı)
	state := float64(1.2) // float64 |  (isteğe bağlı)
	skip := float64(1.2) // float64 |  (isteğe bağlı)
	limit := float64(1.2) // float64 |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTickets(context.Background()).TenantId(tenantId).UserId(userId).State(state).Skip(skip).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTickets``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetTickets`'ten gelen yanıt: GetTickets200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTickets`: %v\n", resp)
}
[inline-code-end]