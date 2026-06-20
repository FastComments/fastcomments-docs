## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | sorgu | Evet |  |
| domain | string | yol | Evet |  |

## Cevap

Döndürür: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_domain_config_response.go)

## Örnek

[inline-code-attrs-start title = 'GetDomainConfig Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	domain := "domain_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetDomainConfig(context.Background(), domain).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetDomainConfig``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetDomainConfig`'den gelen yanıt: GetDomainConfigResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetDomainConfig`: %v\n", resp)
}
[inline-code-end]