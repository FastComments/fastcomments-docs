## Parametri

| Nome | Tipo | Posizione | Richiesto | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| skip | number | query | No |  |

## Risposta

Restituisce: [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tenant_packages_200_response.go)

## Esempio

[inline-code-attrs-start title = 'Esempio di GetTenantPackages'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	skip := float64(1.2) // float64 |  (opzionale)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTenantPackages(context.Background()).TenantId(tenantId).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTenantPackages``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// risposta da `GetTenantPackages`: GetTenantPackages200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTenantPackages`: %v\n", resp)
}
[inline-code-end]