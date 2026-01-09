---
## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| domainToUpdate | string | path | Tak |  |

## Odpowiedź

Zwraca: [`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_domain_config_200_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład PatchDomainConfig'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	domainToUpdate := "domainToUpdate_example" // string | 
	patchDomainConfigParams := *openapiclient.NewPatchDomainConfigParams() // PatchDomainConfigParams | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PatchDomainConfig(context.Background(), domainToUpdate).TenantId(tenantId).PatchDomainConfigParams(patchDomainConfigParams).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.PatchDomainConfig``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `PatchDomainConfig`: GetDomainConfig200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PatchDomainConfig`: %v\n", resp)
}
[inline-code-end]

---