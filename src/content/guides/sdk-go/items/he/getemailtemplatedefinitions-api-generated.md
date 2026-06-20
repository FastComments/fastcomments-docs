---
## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |

## תגובה

מחזיר: [`GetEmailTemplateDefinitionsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_email_template_definitions_response.go)

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
	tenantId := "tenantId_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetEmailTemplateDefinitions(context.Background()).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetEmailTemplateDefinitions``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ־`GetEmailTemplateDefinitions`: GetEmailTemplateDefinitionsResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetEmailTemplateDefinitions`: %v\n", resp)
}
[inline-code-end]

---