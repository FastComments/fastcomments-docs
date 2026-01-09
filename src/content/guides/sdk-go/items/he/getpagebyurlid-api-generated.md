## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| urlId | string | query | כן |  |

## תגובה

מחזיר: [`GetPageByURLIdAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_page_by_urlid_api_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-GetPageByURLId'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	// תגובה מ-`GetPageByURLId`: GetPageByURLIdAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetPageByURLId`: %v\n", resp)
}
[inline-code-end]