## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |

## 回應

回傳：[`AddDomainConfig200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_add_domain_config_200_response.go)

## 範例

[inline-code-attrs-start title = 'AddDomainConfig 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	addDomainConfigParams := *openapiclient.NewAddDomainConfigParams("Domain_example") // AddDomainConfigParams | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.AddDomainConfig(context.Background()).TenantId(tenantId).AddDomainConfigParams(addDomainConfigParams).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.AddDomainConfig``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 從 `AddDomainConfig` 的回應：AddDomainConfig200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.AddDomainConfig`: %v\n", resp)
}
[inline-code-end]

---