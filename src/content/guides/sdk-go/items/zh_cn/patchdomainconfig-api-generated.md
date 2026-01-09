## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| domainToUpdate | string | path | 是 |  |

## 响应

返回: [`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_domain_config_200_response.go)

## 示例

[inline-code-attrs-start title = 'PatchDomainConfig 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	// response from `PatchDomainConfig`: GetDomainConfig200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PatchDomainConfig`: %v\n", resp)
}
[inline-code-end]

---