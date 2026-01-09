## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| domain | string | path | Evet |  |

## Yanıt

Döndürür: [`DeleteDomainConfig200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_delete_domain_config_200_response.go)

## Örnek

[inline-code-attrs-start title = 'DeleteDomainConfig Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	// `DeleteDomainConfig` çağrısından gelen yanıt: DeleteDomainConfig200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeleteDomainConfig`: %v\n", resp)
}
[inline-code-end]