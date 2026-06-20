---
## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| domainToUpdate | string | path | Tak |  |

## Odpowiedź

Zwraca: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_patch_domain_config_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład PatchDomainConfig'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	domainToUpdate := "domainToUpdate_example" // string | 
	patchDomainConfigParams := *openapiclient.NewPatchDomainConfigParams() // PatchDomainConfigParams | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PatchDomainConfig(context.Background(), domainToUpdate).TenantId(tenantId).PatchDomainConfigParams(patchDomainConfigParams).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.PatchDomainConfig``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `PatchDomainConfig`: PatchDomainConfigResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PatchDomainConfig`: %v\n", resp)
}
[inline-code-end]

---