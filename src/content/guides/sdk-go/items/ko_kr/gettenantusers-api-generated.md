## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| skip | number | query | 아니오 |  |

## 응답

반환: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tenant_users_response.go)

## 예제

[inline-code-attrs-start title = 'GetTenantUsers 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	skip := float64(1.2) // float64 |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTenantUsers(context.Background()).TenantId(tenantId).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTenantUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetTenantUsers`의 응답: GetTenantUsersResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTenantUsers`: %v\n", resp)
}
[inline-code-end]

---