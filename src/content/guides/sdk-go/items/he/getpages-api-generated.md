## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |

## תגובה

מחזיר: [`GetPagesAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_pages_api_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-GetPages'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // מחרוזת | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetPages(context.Background()).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetPages``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`GetPages`: GetPagesAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetPages`: %v\n", resp)
}
[inline-code-end]

---