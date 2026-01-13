## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| skip | number | query | Nie |  |

## Odpowiedź

Zwraca: [`GetTenantUsers200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tenant_users_200_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład GetTenantUsers'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	skip := float64(1.2) // float64 |  (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTenantUsers(context.Background()).TenantId(tenantId).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTenantUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `GetTenantUsers`: GetTenantUsers200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTenantUsers`: %v\n", resp)
}
[inline-code-end]

---