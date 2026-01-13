## Parametri

| Nome | Tipo | Posizione | Richiesto | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| id | string | path | Sì |  |

## Risposta

Restituisce: [`GetTenant200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tenant_200_response.go)

## Esempio

[inline-code-attrs-start title = 'Esempio GetTenant'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTenant(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTenant``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// risposta da `GetTenant`: GetTenant200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTenant`: %v\n", resp)
}
[inline-code-end]

---