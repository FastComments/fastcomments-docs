## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |

## Yanıt

Dönüş: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_flag_comment_public_200_response.go)

## Örnek

[inline-code-attrs-start title = 'ReplaceTenantPackage Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 
	replaceTenantPackageBody := *openapiclient.NewReplaceTenantPackageBody("Name_example", float64(123), float64(123), float64(123), float64(123), float64(123), float64(123), float64(123), float64(123), float64(123), float64(123), false, "ForWhoText_example", []string{"FeatureTaglines_example"}, false) // ReplaceTenantPackageBody | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.ReplaceTenantPackage(context.Background(), id).TenantId(tenantId).ReplaceTenantPackageBody(replaceTenantPackageBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.ReplaceTenantPackage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ReplaceTenantPackage`: FlagCommentPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.ReplaceTenantPackage`: %v\n", resp)
}
[inline-code-end]