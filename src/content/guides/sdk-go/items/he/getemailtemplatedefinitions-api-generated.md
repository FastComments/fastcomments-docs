## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |

## תגובה

מחזיר: [`GetEmailTemplateDefinitions200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_email_template_definitions_200_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-GetEmailTemplateDefinitions'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // מחרוזת | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetEmailTemplateDefinitions(context.Background()).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetEmailTemplateDefinitions``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`GetEmailTemplateDefinitions`: GetEmailTemplateDefinitions200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetEmailTemplateDefinitions`: %v\n", resp)
}
[inline-code-end]

---