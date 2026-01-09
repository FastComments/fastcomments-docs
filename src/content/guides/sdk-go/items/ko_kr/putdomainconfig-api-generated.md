## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| domainToUpdate | string | path | 예 |  |

## 응답

반환: [`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_domain_config_200_response.go)

## 예제

[inline-code-attrs-start title = 'PutDomainConfig 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	// `PutDomainConfig`의 응답: GetDomainConfig200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PutDomainConfig`: %v\n", resp)
}
[inline-code-end]