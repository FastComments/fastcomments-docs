## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| domain | string | path | Tak |  |

## Odpowiedź

Zwraca: [`DeleteDomainConfig200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_delete_domain_config_200_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład DeleteDomainConfig'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	domain := "domain_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.DeleteDomainConfig(context.Background(), domain).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeleteDomainConfig``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `DeleteDomainConfig`: DeleteDomainConfig200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeleteDomainConfig`: %v\n", resp)
}
[inline-code-end]

---