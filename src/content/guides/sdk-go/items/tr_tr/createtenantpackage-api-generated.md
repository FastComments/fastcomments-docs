## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |

## Yanıt

Döndürür: [`CreateTenantPackageResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_tenant_package_response.go)

## Örnek

[inline-code-attrs-start title = 'CreateTenantPackage Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createTenantPackageBody := *openapiclient.NewCreateTenantPackageBody("Name_example", float64(123), float64(123), float64(123), float64(123), float64(123), float64(123), float64(123), float64(123), false, "ForWhoText_example", []string{"FeatureTaglines_example"}, false) // CreateTenantPackageBody | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateTenantPackage(context.Background()).TenantId(tenantId).CreateTenantPackageBody(createTenantPackageBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateTenantPackage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `CreateTenantPackage`'dan gelen yanıt: CreateTenantPackageResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateTenantPackage`: %v\n", resp)
}
[inline-code-end]