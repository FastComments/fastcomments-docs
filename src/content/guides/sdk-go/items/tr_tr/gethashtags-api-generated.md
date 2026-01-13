## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| page | number | query | Hayır |  |

## Yanıt

Döndürür: [`GetHashTags200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_hash_tags_200_response.go)

## Örnek

[inline-code-attrs-start title = 'GetHashTags Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := float64(1.2) // float64 |  (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetHashTags(context.Background()).TenantId(tenantId).Page(page).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetHashTags``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetHashTags`'den dönen yanıt: GetHashTags200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetHashTags`: %v\n", resp)
}
[inline-code-end]