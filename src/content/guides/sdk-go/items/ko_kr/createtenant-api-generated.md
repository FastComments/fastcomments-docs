## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |

## 응답

반환: [`CreateTenant200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_tenant_200_response.go)

## 예제

[inline-code-attrs-start title = 'CreateTenant 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
    "time"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	createTenantBody := *openapiclient.NewCreateTenantBody("Name_example", []openapiclient.APIDomainConfiguration{*openapiclient.NewAPIDomainConfiguration("Id_example", "Domain_example", time.Now())}) // CreateTenantBody | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateTenant(context.Background()).TenantId(tenantId).CreateTenantBody(createTenantBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateTenant``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `CreateTenant`의 응답: CreateTenant200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateTenant`: %v\n", resp)
}
[inline-code-end]