## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| domainToUpdate | string | path | 是 |  |

## 回應

回傳: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_patch_domain_config_response.go)

## 範例

[inline-code-attrs-start title = 'PatchDomainConfig 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // 字串 | 
	domainToUpdate := "domainToUpdate_example" // 字串 | 
	patchDomainConfigParams := *openapiclient.NewPatchDomainConfigParams() // PatchDomainConfigParams | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PatchDomainConfig(context.Background(), domainToUpdate).TenantId(tenantId).PatchDomainConfigParams(patchDomainConfigParams).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.PatchDomainConfig``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 來自 `PatchDomainConfig` 的回應：PatchDomainConfigResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PatchDomainConfig`: %v\n", resp)
}
[inline-code-end]