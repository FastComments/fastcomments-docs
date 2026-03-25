## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| userId | string | query | Evet |  |

## Yanıt

Döndürür: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_ticket_200_response.go)

## Örnek

[inline-code-attrs-start title = 'CreateTicket Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	// `CreateTicket`'ten dönen yanıt: CreateTicket200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateTicket`: %v\n", resp)
}
[inline-code-end]