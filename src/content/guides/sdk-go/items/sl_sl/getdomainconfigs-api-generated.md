---
## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |

## Odgovor

Vrne: [`GetDomainConfigsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_domain_configs_response.go)

## Primer

[inline-code-attrs-start title = 'Primer GetDomainConfigs'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetDomainConfigs(context.Background()).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetDomainConfigs``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor iz `GetDomainConfigs`: GetDomainConfigsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetDomainConfigs`: %v\n", resp)
}
[inline-code-end]

---