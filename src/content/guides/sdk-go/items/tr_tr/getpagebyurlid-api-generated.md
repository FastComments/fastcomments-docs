## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | sorgu | Evet |  |
| urlId | string | sorgu | Evet |  |

## Yanıt

Döndürür: [`GetPageByURLIdAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_page_by_urlid_api_response.go)

## Örnek

[inline-code-attrs-start title = 'GetPageByURLId Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetPageByURLId(context.Background()).TenantId(tenantId).UrlId(urlId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetPageByURLId``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetPageByURLId` yanıtı: GetPageByURLIdAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetPageByURLId`: %v\n", resp)
}
[inline-code-end]