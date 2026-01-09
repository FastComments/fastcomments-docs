## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| domainToUpdate | string | path | Evet |  |

## Yanıt

Döndürür: [`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_domain_config_200_response.go)

## Örnek

[inline-code-attrs-start title = 'PutDomainConfig Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	updateDomainConfigParams := *openapiclient.NewUpdateDomainConfigParams("Domain_example") // UpdateDomainConfigParams | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PutDomainConfig(context.Background(), domainToUpdate).TenantId(tenantId).UpdateDomainConfigParams(updateDomainConfigParams).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.PutDomainConfig``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `PutDomainConfig`'den yanıt: GetDomainConfig200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PutDomainConfig`: %v\n", resp)
}
[inline-code-end]