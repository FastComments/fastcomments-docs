## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## 回應

回傳: [`AddDomainConfigResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_add_domain_config_response.go)

## 範例

[inline-code-attrs-start title = 'AddDomainConfig 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	addDomainConfigParams := *openapiclient.NewAddDomainConfigParams("Domain_example") // AddDomainConfigParams | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.AddDomainConfig(context.Background()).TenantId(tenantId).AddDomainConfigParams(addDomainConfigParams).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.AddDomainConfig``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 來自 `AddDomainConfig` 的回應: AddDomainConfigResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.AddDomainConfig`: %v\n", resp)
}
[inline-code-end]