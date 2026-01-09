## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |

## 응답

반환: [`GetDomainConfigs200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_domain_configs_200_response.go)

## 예제

[inline-code-attrs-start title = 'GetDomainConfigs 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // 문자열 | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetDomainConfigs(context.Background()).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetDomainConfigs``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetDomainConfigs`의 응답: GetDomainConfigs200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetDomainConfigs`: %v\n", resp)
}
[inline-code-end]

---