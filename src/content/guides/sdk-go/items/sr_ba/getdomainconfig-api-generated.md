## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| domain | string | path | Yes |  |

## Одговор

Враћа: [`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_domain_config_200_response.go)

## Пример

[inline-code-attrs-start title = 'GetDomainConfig Пример'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	resp, r, err := apiClient.DefaultAPI.GetDomainConfig(context.Background(), domain).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetDomainConfig``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// одговор од `GetDomainConfig`: GetDomainConfig200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetDomainConfig`: %v\n", resp)
}
[inline-code-end]